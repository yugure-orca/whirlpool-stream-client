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
enum RawMessage {
    #[serde(rename = "opened")]
    Opened,
    #[serde(rename = "closed")]
    Closed { reason: String },
    #[serde(rename = "nodata")]
    Nodata,
    #[serde(rename = "data.event")]
    DataEvent { data: serde_json::Value },
    #[serde(rename = "data.account")]
    DataAccount { data: serde_json::Value },
}

#[derive(Debug)]
pub enum WhirlpoolStreamMessage {
    Heartbeat,
    Data { slot: u64, block_height: u64, block_time: u64, event: Vec<serde_json::Value>, account: Vec<serde_json::Value> },
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
        
        let mut client = Self { endpoint, apikey, slot, limit, event, account, reader };

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
    pub async fn next(&mut self) -> Option<Result<String, Box<dyn Error>>> {
        let receive_event = !matches!(self.event, EventParam::None);
        let receive_account = !matches!(self.account, AccountParam::None);

        let event = if receive_event || receive_account {
            Some(self.read_message().await?)
        } else {
            None
        };

        // error check

        let account = if receive_account {
            Some(self.read_message().await?)
        } else {
            None
        };

        // error check

        let event = event.unwrap();
        let account = account.unwrap();

        // TODO: height check

        let response = format!("\nevent: {}\naccount: {}",
            event.map(|e| e.chars().take(128).collect::<String>()).unwrap_or("".to_string()),
            account.map(|a| a.chars().take(128).collect::<String>()).unwrap_or("".to_string()));
        Some(Ok(response))
    }

    async fn read_message(&mut self) -> Option<Result<String, Box<dyn Error>>> {
        // サーバー側は5秒以上無応答であるべきではない
        let timeout_duration = Duration::from_secs(10);

        match timeout(timeout_duration, self.reader.next()).await {
            Ok(Some(Ok(Message::Text(text)))) => {
                let preview = text.chars().take(128).collect::<String>();
                Some(Ok(preview))
            },
            Ok(Some(Ok(Message::Close(_)))) => None,
            Ok(Some(Ok(_))) => Some(Ok("非テキストメッセージを受信".to_string())),
            Ok(Some(Err(e))) => Some(Err(Box::new(e))),
            Ok(None) => None,
            Err(_) => Some(Err("タイムアウトが発生しました".into())),
        }
    }
}

// 使用例
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut client = WhirlpoolStreamWebsocketClient::connect(
        "wss://orcanauts-a.whirlpool-stream.pleiades.dev".to_string(),
        "demo".to_string(),
        None,
        Some(50),
        EventParam::Trade,
        AccountParam::All,
    ).await?;
    
    // シンプルな受信ループ
    while let Some(message) = client.next().await {
        match message {
            Ok(text) => println!("受信: {}...", text),
            Err(e) => eprintln!("エラー: {}", e),
        }
    }

    Ok(())
}
