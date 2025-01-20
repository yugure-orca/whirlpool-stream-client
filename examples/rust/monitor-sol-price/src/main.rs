use std::error::Error;
use clap::Parser;
use whirlpool_stream_websocket_client::{WhirlpoolStreamWebsocketClient, WhirlpoolStreamMessage, EventParam, AccountParam, event::WhirlpoolEvent, event::definition::TradeDirection};

#[derive(Parser)]
struct Args {
    #[arg(short = 'e', long, default_value = "wss://orcanauts-a.whirlpool-stream.pleiades.dev")]
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
    ).await?;
    
    while let Some(message) = client.next().await {
        match message {
            Ok(WhirlpoolStreamMessage::Data { slot, block_height, block_time, events, .. }) => {
                println!(
                    "Data: slot:{}, height:{}, time:{}", 
                    slot, 
                    block_height, 
                    block_time, 
                );
                for transaction in events {
                    for event in transaction.events {
                        match event {
                            WhirlpoolEvent::Traded(payload) => {
                                // SOL/USDC(ts=4) is most traded pool, so we use it to monitor SOL price
                                if payload.whirlpool == "Czfq3xZZDmsdGdUyrNLtRhGc47cXcZtLG4crryfu44zE" {
                                    match payload.trade_direction {
                                        TradeDirection::AtoB => {
                                            println!("\tSOL price (💧): {}, {} SOL has been sold", payload.new_decimal_price, payload.transfer_in.amount as f64 / 1_000_000_000.0);
                                        },
                                        TradeDirection::BtoA => {
                                            println!("\tSOL price (🔥): {}, {} SOL has been bought", payload.new_decimal_price, payload.transfer_out.amount as f64 / 1_000_000_000.0);
                                        },
                                    }
                                }
                            },
                            _ => { /* ignore */ },
                        }
                    }
                }
            },
            Ok(WhirlpoolStreamMessage::Heartbeat) => println!("Heartbeat"),
            Ok(WhirlpoolStreamMessage::Closed { reason }) => println!("Closed: {}", reason),
            Err(e) => eprintln!("ERROR: {}", e),
        }
    }

    Ok(())
}
