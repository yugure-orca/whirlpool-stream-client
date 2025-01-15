use std::error::Error;
use whirlpool_stream_websocket_client::{WhirlpoolStreamWebsocketClient, WhirlpoolStreamMessage, EventParam, AccountParam};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut client = WhirlpoolStreamWebsocketClient::connect(
        "wss://orcanauts-a.whirlpool-stream.pleiades.dev",
        "demo",
        None,
        Some(50), // This setting does not affect the stream being disconnected once a day. Client must implement reconnection logic.
        EventParam::All,
        AccountParam::All,
    ).await?;
    
    while let Some(message) = client.next().await {
        match message {
            Ok(WhirlpoolStreamMessage::Data { slot, block_height, block_time, events, accounts }) => {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i64;
                let time_diff = now - block_time;
                
                println!(
                    "Data: slot:{}, height:{}, time:{} ({}s ago), events:{}, accounts:{}", 
                    slot, 
                    block_height, 
                    block_time, 
                    time_diff,
                    events.len(), 
                    accounts.len()
                );
            },
            Ok(WhirlpoolStreamMessage::Heartbeat) => println!("Heartbeat"),
            Ok(WhirlpoolStreamMessage::Closed { reason }) => println!("Closed: {}", reason),
            Err(e) => eprintln!("ERROR: {}", e),
        }
    }

    Ok(())
}
