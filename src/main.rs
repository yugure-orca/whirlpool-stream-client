use futures_util::StreamExt;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use std::error::Error;
use tokio::time::{timeout, Duration};

#[derive(Debug)]
pub enum EventParam {
    None,
    Trade,
    Liquidity,
    All,
}

#[derive(Debug)]
pub enum AccountParam {
    None,
    Trade,
    All,
}

#[derive(Debug, serde_derive::Deserialize)]
#[serde(tag = "ctrl")]
enum WebsocketMessage {
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
    // parameters
    endpoint: String,
    apikey: String,
    slot: Option<u64>,
    limit: Option<u32>,
    event: EventParam,
    account: AccountParam,

    // state
    last_received_point: Option<(u64, u64, i64)>, // (slot, block_height, block_time)
    last_received_system_time: Option<i64>,
    received_count: u64,

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
        endpoint: String,
        apikey: String,
        slot: Option<u64>,
        limit: Option<u32>,
        event: EventParam,
        account: AccountParam,
    ) -> Result<Self, Box<dyn Error>> {
        let url = Self::build_url(&endpoint, &apikey, slot, limit, &event, &account)?;

        //let url = Url::parse(url)?;
        println!("built URL: {}", url);
        let (ws_stream, _) = connect_async(url.as_str()).await?;
        let (_, reader) = ws_stream.split();
        
        let mut client = Self { endpoint, apikey, slot, limit, event, account, reader, last_received_point: None, last_received_system_time: None, received_count: 0 };

        let first_message = client.read_message().await
            .ok_or_else(|| "最初のメッセージの読み取りに失敗しました".to_string())??;
        println!("first message: {:?}", first_message);

        Ok(client)
    }

    fn build_url(
        endpoint: &String,
        apikey: &String,
        slot: Option<u64>,
        limit: Option<u32>,
        event: &EventParam,
        account: &AccountParam
    ) -> Result<String, Box<dyn Error>> {
        let mut url = url::Url::parse(format!(
            "{}/{}/stream/refined/ws",
            endpoint.to_string().trim_end_matches('/'),
            apikey,
        ).as_str())?;

        let mut query_params = url.query_pairs_mut();

        if let Some(slot) = slot {
            query_params.append_pair("slot", &slot.to_string());
        }

        if let Some(limit) = limit {
            query_params.append_pair("limit", &limit.to_string());
        }

        match event {
            EventParam::None => {},
            EventParam::Trade => { query_params.append_pair("event", "trade"); },
            EventParam::Liquidity => { query_params.append_pair("event", "liquidity"); },
            EventParam::All => { query_params.append_pair("event", "all"); },
        }

        match account {
            AccountParam::None => {},
            AccountParam::Trade => { query_params.append_pair("account", "trade"); },
            AccountParam::All => { query_params.append_pair("account", "all"); },
        }

        drop(query_params);

        Ok(url.to_string())
    }

    // 次のメッセージを取得
    pub async fn next(&mut self) -> Option<Result<WhirlpoolStreamMessage, Box<dyn Error>>> {
        let receive_event = !matches!(self.event, EventParam::None);
        let receive_account = !matches!(self.account, AccountParam::None);

        let mut event = None;
        if receive_event {
            let message = self.read_message().await;
            if message.is_none() {
                return None;
            }
            let message = message.unwrap();
            if let Err(e) = message {
                return Some(Err(e));
            }
            let message = message.unwrap();

            match message {
                WebsocketMessage::Nodata => { return Some(Ok(WhirlpoolStreamMessage::Heartbeat)); }
                WebsocketMessage::Closed { reason } => { return Some(Ok(WhirlpoolStreamMessage::Closed { reason })); }
                WebsocketMessage::Opened => { return Some(Err("not expected Opened".into())); }
                WebsocketMessage::DataAccount { data } => { return Some(Err("not expected DataAccount".into())); }
                WebsocketMessage::DataEvent { data } => { event = Some(data); }
            }
        };

        // error check

        let mut account = None;
        if receive_account {
            let message = self.read_message().await;
            if message.is_none() {
                return None;
            }
            let message = message.unwrap();
            if let Err(e) = message {
                return Some(Err(e));
            }
            let message = message.unwrap();

            match message {
                WebsocketMessage::Nodata => {
                    if event.is_some() {
                        return Some(Err("not expected Nodata".into()));
                    }
                    return Some(Ok(WhirlpoolStreamMessage::Heartbeat));
                }
                WebsocketMessage::Closed { reason } => { return Some(Ok(WhirlpoolStreamMessage::Closed { reason })); }
                WebsocketMessage::Opened => { return Some(Err("not expected Opened".into())); }
                WebsocketMessage::DataEvent { data } => { return Some(Err("not expected DataEvent".into())); }
                WebsocketMessage::DataAccount { data } => { account = Some(data); }
            }
        };

        //self.last_received_point = Some((slot, block_height, block_time));
        //self.last_received_system_time = Some(block_time);
        //self.received_count += 1;

        let (slot, block_height, block_time, events, accounts) = match (event, account) {
            (Some(event), Some(account)) => {
                assert_eq!(event.slot, account.slot);
                assert_eq!(event.block_height, account.block_height);
                assert_eq!(event.block_time, account.block_time);
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

    async fn read_message(&mut self) -> Option<Result<WebsocketMessage, Box<dyn Error>>> {
        let timeout_duration = Duration::from_secs(10);

        let message = timeout(timeout_duration, self.reader.next()).await;
        match message {
            Err(elapsed) => Some(Err(Box::new(elapsed))),
            Ok(message) => {
                match message {
                    // 閉じる
                    None => None,
                    // エラー
                    Some(Err(e)) => Some(Err(Box::new(e))),
                    // Websocketのコネクションが切断されたので終了
                    Some(Ok(Message::Close(_))) => None,
                    // テキストメッセージを受信
                    Some(Ok(Message::Text(text))) => {
                        // JSONテキストをRawMessageにパースする
                        match serde_json::from_str(&text) {
                            Ok(raw_message) => Some(Ok(raw_message)),
                            Err(e) => Some(Err(Box::new(e))),
                        }
                    },
                    // 非テキストメッセージを受信
                    Some(Ok(_)) => Some(Err("非テキストメッセージを受信".into())),
                }    
            }
        }
    }

    // TODO: close
}

// 使用例
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut client = WhirlpoolStreamWebsocketClient::connect(
        "wss://orcanauts-a.whirlpool-stream.pleiades.dev".to_string(),
        "demo".to_string(),
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
