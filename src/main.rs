use futures_util::StreamExt;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use std::error::Error;
use tokio::time::{timeout, Duration};

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
    DataEvent { data: DataEvent },
    #[serde(rename = "data.account")]
    DataAccount { data: DataAccount },
}

#[derive(Debug, serde_derive::Deserialize)]
struct DataEvent {
    #[serde(rename = "s")]
    slot: u64,
    #[serde(rename = "h")]
    block_height: u64,
    #[serde(rename = "t")]
    block_time: i64,
    #[serde(rename = "x")]
    events: Vec<serde_json::Value>,
}

#[derive(Debug, serde_derive::Deserialize)]
struct DataAccount {
    #[serde(rename = "s")]
    slot: u64,
    #[serde(rename = "h")]
    block_height: u64,
    #[serde(rename = "t")]
    block_time: i64,
    #[serde(rename = "a")]
    accounts: Vec<serde_json::Value>,
}

#[derive(Debug)]
pub enum WhirlpoolStreamMessage {
    Heartbeat,
    Data { slot: u64, block_height: u64, block_time: i64, events: Vec<serde_json::Value>, accounts: Vec<serde_json::Value> },
    Closed { reason: String },
}

#[derive(Debug, thiserror::Error)]
enum WhirlpoolStreamError {
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
    #[error("Other error: {0}")]
    Other(Box<dyn Error>),
}

// Message
// 正常系
// - Heartbeat
// - Data
// 異常系
// - Closed
// - Error
// - Timeout





// WebSocketクライアントをラップする構造体
pub struct WhirlpoolStreamWebsocketClient {
    event: EventParam,
    account: AccountParam,

    // state
    last_received_point: Option<(u64, u64, i64)>, // (slot, block_height, block_time)
    last_received_system_time: Option<i64>,
    received_count: u64,

    is_closed: bool,

    // stream
    reader: futures_util::stream::SplitStream<
        tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>
        >
    >,
}

impl WhirlpoolStreamWebsocketClient {

    // 新しいクライアントを作成
    pub async fn connect(
        endpoint: &str,
        apikey: &str,
        slot: Option<u64>,
        limit: Option<u32>,
        event: EventParam,
        account: AccountParam,
    ) -> Result<Self, Box<dyn Error>> {
        let endpoint = endpoint.to_string();
        let apikey = apikey.to_string();

        let mut url = url::Url::parse(&format!(
            "{}/{}/stream/refined/ws",
            endpoint.trim_end_matches('/'),
            apikey,
        ))?;

        let mut query_params = url.query_pairs_mut();
        if let Some(slot) = slot {
            query_params.append_pair("slot", &slot.to_string());
        }
        if let Some(limit) = limit {
            query_params.append_pair("limit", &limit.to_string());
        }
        query_params.append_pair("event", event.as_query_param());
        query_params.append_pair("account", account.as_query_param());
        drop(query_params);

        let (ws_stream, _) = connect_async(url.as_str()).await?;
        let (_, reader) = ws_stream.split();
        
        let mut client = Self { event, account, reader, last_received_point: None, last_received_system_time: None, received_count: 0, is_closed: false };

        let first_message = client.read().await
            .ok_or_else(|| "最初のメッセージの読み取りに失敗しました".to_string())??;
        println!("first message: {:?}", first_message);

        Ok(client)
    }

    // 次のメッセージを取得
    pub async fn next(&mut self) -> Option<Result<WhirlpoolStreamMessage, WhirlpoolStreamError>> {
        let receive_event = !matches!(self.event, EventParam::None);
        let event = if receive_event {
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

        let receive_account = !matches!(self.account, AccountParam::None);
        let account = if receive_account {
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

        //self.last_received_point = Some((slot, block_height, block_time));
        //self.last_received_system_time = Some(block_time);
        //self.received_count += 1;

        let (slot, block_height, block_time, events, accounts) = match (event, account) {
            (Some(event), Some(account)) => {
                if event.slot != account.slot || event.block_height != account.block_height || event.block_time != account.block_time {
                    return Some(Err(WhirlpoolStreamError::InconsistentMessage));
                }
                (event.slot, event.block_height, event.block_time, event.events, account.accounts)
            }
            (Some(event), None) => (event.slot, event.block_height, event.block_time, event.events, vec![]),
            (None, Some(account)) => (account.slot, account.block_height, account.block_time, vec![], account.accounts),
            (None, None) => unreachable!(),
        };
                
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

// EventParamの拡張
impl EventParam {
    fn as_query_param(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Trade => "trade",
            Self::Liquidity => "liquidity",
            Self::All => "all",
        }
    }
}

// AccountParamの拡張
impl AccountParam {
    fn as_query_param(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Trade => "trade",
            Self::All => "all",
        }
    }
}

// 使用例
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut client = WhirlpoolStreamWebsocketClient::connect(
        "wss://orcanauts-a.whirlpool-stream.pleiades.dev",
        "demo",
        None,
        Some(500000),
        EventParam::Trade,
        AccountParam::All,
    ).await?;
    
    // シンプルな受信ループ
    while let Some(message) = client.next().await {
        match message {
            Ok(text) => println!("受信: {:?}...", text),
            Err(e) => eprintln!("エラー: {}", e),
        }
    }

    Ok(())
}

