use futures_util::StreamExt;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "wss://orcanauts-a.whirlpool-stream.pleiades.dev/demo/stream/refined/ws?limit=50000";
    // サーバーに接続
    let (ws_stream, _) = connect_async(url).await?;
    println!("WebSocket接続が確立されました");

    // 送受信ストリームを分割
    let (_, mut read) = ws_stream.split();

    // メッセージの受信ループ
    while let Some(message) = read.next().await {
        match message {
            Ok(msg) => {
                match msg {
                    Message::Text(text) => {
                        let preview = text.chars().take(100).collect::<String>();
                        println!("テキストメッセージを受信: {}...", preview);
                    },
                    Message::Binary(data) => println!("バイナリメッセージを受信: {} bytes", data.len()),
                    Message::Ping(_) => println!("Pingを受信"),
                    Message::Pong(_) => println!("Pongを受信"),
                    Message::Close(_) => {
                        println!("接続が閉じられました");
                        break;
                    },
                    Message::Frame(_) => println!("フレームを受信"),
                }
            },
            Err(e) => {
                eprintln!("エラーが発生しました: {}", e);
                break;
            }
        }
    }

    Ok(())
}
