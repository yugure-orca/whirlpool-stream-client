use clap::Parser;
use std::error::Error;
use whirlpool_stream_websocket_client::{
    event::WhirlpoolEvent, AccountParam, EventParam, WhirlpoolStreamMessage,
    WhirlpoolStreamWebsocketClient,
};

#[derive(Parser)]
struct Args {
    #[arg(
        short = 'e',
        long,
        default_value = "wss://orcanauts-a.whirlpool-stream.pleiades.dev"
    )]
    endpoint: String,
    #[arg(short = 'k', long, default_value = "demo")]
    apikey: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut client = WhirlpoolStreamWebsocketClient::connect(
        args.endpoint.as_str(),
        args.apikey.as_str(),
        None,
        Some(500), // This setting does not affect the stream being disconnected once a day. Client must implement reconnection logic.
        EventParam::Trade, // Traded event only
        AccountParam::None, // no account delta
    )
    .await?;

    while let Some(message) = client.next().await {
        match message {
            Ok(WhirlpoolStreamMessage::Data {
                slot,
                block_height,
                block_time,
                events,
                ..
            }) => {
                println!(
                    "Data: slot:{}, height:{}, time:{}",
                    slot, block_height, block_time,
                );
                for transaction in events {
                    for event in transaction.events {
                        match event {
                            WhirlpoolEvent::Traded(payload) => {
                                println!("\tTraded pool: {}, direction: {:?}, in: {}, out: {}, price: {} -> {}, payer: {}",
                                payload.whirlpool,
                                payload.trade_direction,
                                payload.transfer_in.amount,
                                payload.transfer_out.amount,
                                payload.old_decimal_price,
                                payload.new_decimal_price,
                                transaction.payer,
                            );
                            }
                            _ => { /* ignore */ }
                        }
                    }
                }
            }
            Ok(WhirlpoolStreamMessage::Heartbeat) => println!("Heartbeat"),
            Ok(WhirlpoolStreamMessage::Closed { reason }) => println!("Closed: {}", reason),
            Err(e) => eprintln!("ERROR: {}", e),
        }
    }

    Ok(())
}
