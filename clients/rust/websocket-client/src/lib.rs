use futures_util::StreamExt;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use std::error::Error;
use tokio::time::{timeout, Duration};

use whirlpool_stream_data_schema::account_delta::{BlockWhirlpoolAccountDelta, WhirlpoolAccountDelta};
use whirlpool_stream_data_schema::event::{BlockWhirlpoolEvent, TransactionWhirlpoolEvent};

pub use whirlpool_stream_data_schema::account_delta;
pub use whirlpool_stream_data_schema::event;

#[derive(Debug, Clone, Copy)]
pub enum EventParam {
    None,
    Trade,
    Liquidity,
    All,
}

#[derive(Debug, Clone, Copy)]
pub enum AccountParam {
    None,
    Trade,
    All,
}

#[derive(Debug, thiserror::Error)]
pub enum WhirlpoolStreamConnectError {
    #[error("Invalid URL")]
    InvalidUrl,
    #[error("Invalid parameters")]
    InvalidParameters,
    #[error("Connection failed")]
    ConnectFailed,
    #[error("Invalid first message")]
    InvalidFirstMessage,
    #[error("Other error: {0}")]
    Other(Box<dyn Error>),
}

#[derive(Debug)]
pub enum WhirlpoolStreamMessage {
    Heartbeat,
    Data { slot: u64, block_height: u64, block_time: i64, events: Vec<TransactionWhirlpoolEvent>, accounts: Vec<WhirlpoolAccountDelta> },
    Closed { reason: String },
}

#[derive(Debug, thiserror::Error)]
pub enum WhirlpoolStreamError {
    #[error("Connection timeout")]
    Timeout,
    #[error("Received invalid message type")]
    InvalidMessageType,
    #[error("Invalid message format: {0}")]
    InvalidMessageFormat(String),
    #[error("Unexpected message")]
    UnexpectedMessage,
    #[error("Inconsistent message")]
    InconsistentMessage,
    #[error("Non-continuous block height")]
    NonContinuousBlockHeight,
    #[error("Other error: {0}")]
    Other(Box<dyn Error>),
}

#[derive(Debug, serde_derive::Deserialize)]
#[serde(tag = "ctrl")]
enum StreamMessage {
    #[serde(rename = "opened")]
    Opened,
    #[serde(rename = "closed")]
    Closed { reason: String },
    #[serde(rename = "nodata")]
    Nodata,
    #[serde(rename = "data.event")]
    DataEvent { data: BlockWhirlpoolEvent },
    #[serde(rename = "data.account")]
    DataAccount { data: BlockWhirlpoolAccountDelta },
}

#[derive(Debug, Clone, Copy)]
enum ClientState {
    Normal,
    Error,
    Closed,
}

pub struct WhirlpoolStreamWebsocketClient {
    // client state
    state: ClientState,
    // parameters
    receive_event: bool,
    receive_account: bool,
    // stream state
    last_received_block_height: Option<u64>,
    reader: futures_util::stream::SplitStream<
        tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>
        >
    >,
}

impl WhirlpoolStreamWebsocketClient {
    pub async fn connect(
        endpoint: &str,
        apikey: &str,
        slot: Option<u64>,
        limit: Option<u32>,
        event: EventParam,
        account: AccountParam,
    ) -> Result<Self, WhirlpoolStreamConnectError> {
        let receive_event = !matches!(event, EventParam::None);
        let receive_account = !matches!(account, AccountParam::None);
        if !receive_event && !receive_account {
            return Err(WhirlpoolStreamConnectError::InvalidParameters);
        }

        let mut url = url::Url::parse(&format!(
            "{}/{}/stream/refined/ws",
            endpoint.trim_end_matches('/'),
            apikey,
        )).map_err(|_| WhirlpoolStreamConnectError::InvalidUrl)?;

        let mut query_params = url.query_pairs_mut();
        if let Some(slot) = slot {
            query_params.append_pair("slot", &slot.to_string());
        }
        if let Some(limit) = limit {
            query_params.append_pair("limit", &limit.to_string());
        }
        query_params.append_pair("event", match event {
            EventParam::None => "none",
            EventParam::Trade => "trade",
            EventParam::Liquidity => "liquidity",
            EventParam::All => "all",
        });
        query_params.append_pair("account", match account {
            AccountParam::None => "none",
            AccountParam::Trade => "trade",
            AccountParam::All => "all",
        });
        drop(query_params);

        let (ws_stream, _) = connect_async(url.as_str()).await.map_err(|_| WhirlpoolStreamConnectError::ConnectFailed)?;
        let (_, reader) = ws_stream.split();
        
        let mut client = Self { receive_event, receive_account, state: ClientState::Normal, last_received_block_height: None, reader };

        match client.read().await {
            Some(Ok(StreamMessage::Opened)) => Ok(client),
            Some(Ok(_)) => Err(WhirlpoolStreamConnectError::InvalidFirstMessage),
            Some(Err(e)) => Err(WhirlpoolStreamConnectError::Other(e.into())),
            None => Err(WhirlpoolStreamConnectError::InvalidFirstMessage),
        }
    }

    pub async fn next(&mut self) -> Option<Result<WhirlpoolStreamMessage, WhirlpoolStreamError>> {
        if !matches!(self.state, ClientState::Normal) {
            return None;
        }

        let result = self._next().await;
        match &result {
            None => self.state = ClientState::Closed,
            Some(Err(_)) => self.state = ClientState::Error,
            Some(Ok(_)) => {},
        }

        result
    }

    pub async fn _next(&mut self) -> Option<Result<WhirlpoolStreamMessage, WhirlpoolStreamError>> {
        let event = if self.receive_event {
            let message = match self.read().await {
                None => return None,
                Some(Err(e)) => return Some(Err(e)),
                Some(Ok(msg)) => msg,
            };
            match message {
                StreamMessage::Nodata => return Some(Ok(WhirlpoolStreamMessage::Heartbeat)),
                StreamMessage::Closed { reason } => return Some(Ok(WhirlpoolStreamMessage::Closed { reason })),
                StreamMessage::Opened => return Some(Err(WhirlpoolStreamError::UnexpectedMessage)),
                StreamMessage::DataAccount { .. } => return Some(Err(WhirlpoolStreamError::UnexpectedMessage)),
                StreamMessage::DataEvent { data } => Some(data),
            }
        } else {
            None
        };

        let account = if self.receive_account {
            let message = match self.read().await {
                None => return None,
                Some(Err(e)) => return Some(Err(e)),
                Some(Ok(msg)) => msg,
            };
            match message {
                StreamMessage::Nodata => return Some(if event.is_some() {
                    Err(WhirlpoolStreamError::UnexpectedMessage)
                } else {
                    Ok(WhirlpoolStreamMessage::Heartbeat)
                }),
                StreamMessage::Closed { reason } => return Some(if event.is_some() {
                    Err(WhirlpoolStreamError::UnexpectedMessage)
                } else {
                    Ok(WhirlpoolStreamMessage::Closed { reason })
                }),
                StreamMessage::Opened => return Some(Err(WhirlpoolStreamError::UnexpectedMessage)),
                StreamMessage::DataEvent { .. } => return Some(Err(WhirlpoolStreamError::UnexpectedMessage)),
                StreamMessage::DataAccount { data } => Some(data),
            }
        } else {
            None
        };

        let (slot, block_height, block_time, events, accounts) = match (event, account) {
            (Some(event), Some(account)) => {
                if event.slot != account.slot || event.block_height != account.block_height || event.block_time != account.block_time {
                    return Some(Err(WhirlpoolStreamError::InconsistentMessage));
                }
                (event.slot, event.block_height, event.block_time, event.transactions, account.account_deltas)
            }
            (Some(event), None) => (event.slot, event.block_height, event.block_time, event.transactions, vec![]),
            (None, Some(account)) => (account.slot, account.block_height, account.block_time, vec![], account.account_deltas),
            (None, None) => unreachable!(),
        };

        if let Some(last_received_block_height) = self.last_received_block_height {
            if last_received_block_height + 1 != block_height {
                return Some(Err(WhirlpoolStreamError::NonContinuousBlockHeight));
            }
        }
        self.last_received_block_height = Some(block_height);
                
        Some(Ok(WhirlpoolStreamMessage::Data {
            slot,
            block_height,
            block_time,
            events,
            accounts,
        }))
    }

    async fn read(&mut self) -> Option<Result<StreamMessage, WhirlpoolStreamError>> {
        const TIMEOUT: Duration = Duration::from_secs(10);

        let message = timeout(TIMEOUT, self.reader.next()).await;
        match message {
            Err(_) => Some(Err(WhirlpoolStreamError::Timeout)),
            Ok(message) => {
                match message {
                    // already closed
                    None => None,
                    // closed on Websocket layer
                    Some(Ok(Message::Close(_))) => None,
                    // text message
                    Some(Ok(Message::Text(text))) => {
                        match serde_json::from_str(&text) {
                            Ok(message) => Some(Ok(message)),
                            Err(_) => Some(Err(WhirlpoolStreamError::InvalidMessageFormat(text.to_string()))),
                        }
                    },
                    // not text message
                    Some(Ok(_)) => Some(Err(WhirlpoolStreamError::InvalidMessageType)),
                    // error
                    Some(Err(e)) => Some(Err(WhirlpoolStreamError::Other(Box::new(e)))),
                }    
            }
        }
    }
}
