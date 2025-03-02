use clap::Parser;
use std::error::Error;
use whirlpool_stream_websocket_client::{
    event::definition::TradeDirection, event::WhirlpoolEvent, AccountParam, EventParam,
    WhirlpoolStreamMessage, WhirlpoolStreamWebsocketClient,
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
    #[arg(short = 'l', long, default_value = "1000")]
    limit: u32,
    #[arg(short = 'p', long, required = false)]
    pools: Vec<String>,
    // a to b is buy, default is that b to a is buy (USDC to SOL in SOL/USDC)
    #[arg(short = 'a', long)]
    ab: bool,
}

const EVENT_ICON_INITIALIZED: &str = "🎉";
const EVENT_ICON_BUY: &str = "🔥";
const EVENT_ICON_SELL: &str = "💧";
const EVENT_ICON_DEPOSIT: &str = "🔋";
const EVENT_ICON_WITHDRAW: &str = "🪫 ";

const TOKEN_ICON_XBTC: &str = "🟡";
const TOKEN_ICON_XETH: &str = "🔵";
const TOKEN_ICON_SOL: &str = "🟣";
const TOKEN_ICON_XSOL: &str = "🟢";
const TOKEN_ICON_XUSD: &str = "💵";
const TOKEN_ICON_OTHER: &str = "";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut client = WhirlpoolStreamWebsocketClient::connect(
        args.endpoint.as_str(),
        args.apikey.as_str(),
        None,
        Some(args.limit), // This setting does not affect the stream being disconnected once a day. Client must implement reconnection logic.
        EventParam::Liquidity,
        AccountParam::None, // no account delta
    )
    .await?;

    println!("Listening to pools: {:?}", args.pools);

    while let Some(message) = client.next().await {
        match message {
            Ok(WhirlpoolStreamMessage::Data {
                slot,
                block_height,
                block_time,
                events,
                ..
            }) => {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i64;
                let ago = now - block_time;

                println!(
                    "Data: slot:{}, height:{}, time:{} ({}s ago)",
                    slot, block_height, block_time, ago
                );
                for transaction in events {
                    for event in transaction.events {
                        match event {
                            WhirlpoolEvent::Traded(payload) => {
                                if match_pool(&payload.whirlpool, &args.pools) {
                                    let buy_or_sell = if args.ab
                                        == (payload.trade_direction == TradeDirection::AtoB)
                                    {
                                        EVENT_ICON_BUY
                                    } else {
                                        EVENT_ICON_SELL
                                    };
                                    println!(
                                        "\t{} {:<44} Traded direction: {:?}, in: {}{}, out: {}{}, price: {} -> {}, payer: {}",
                                        buy_or_sell,
                                        payload.whirlpool,
                                        payload.trade_direction,
                                        payload.transfer_in.amount,
                                        get_token_icon(&payload.transfer_in.mint),
                                        payload.transfer_out.amount,
                                        get_token_icon(&payload.transfer_out.mint),
                                        payload.old_decimal_price,
                                        payload.new_decimal_price,
                                        transaction.payer,
                                    );
                                }
                            }
                            WhirlpoolEvent::LiquidityDeposited(payload) => {
                                if match_pool(&payload.whirlpool, &args.pools) {
                                    println!(
                                        "\t{} {:<44}  Deposited range: [{}, {}], liquidity: {}, a: {}{}, b: {}{}, payer: {}",
                                        EVENT_ICON_DEPOSIT,
                                        payload.whirlpool,
                                        payload.lower_tick_index,
                                        payload.upper_tick_index,
                                        payload.liquidity_delta,
                                        payload.transfer_a.amount,
                                        get_token_icon(&payload.transfer_a.mint),
                                        payload.transfer_b.amount,
                                        get_token_icon(&payload.transfer_b.mint),
                                        transaction.payer,
                                    );
                                }
                            }
                            WhirlpoolEvent::LiquidityWithdrawn(payload) => {
                                if match_pool(&payload.whirlpool, &args.pools) {
                                    println!(
                                        "\t{} {:<44}  Withdrawn range: [{}, {}], liquidity: {}, a: {}{}, b: {}{}, payer: {}",
                                        EVENT_ICON_WITHDRAW,
                                        payload.whirlpool,
                                        payload.lower_tick_index,
                                        payload.upper_tick_index,
                                        payload.liquidity_delta,
                                        payload.transfer_a.amount,
                                        get_token_icon(&payload.transfer_a.mint),
                                        payload.transfer_b.amount,
                                        get_token_icon(&payload.transfer_b.mint),
                                        transaction.payer,
                                    );
                                }
                            }
                            WhirlpoolEvent::PoolInitialized(payload) => {
                                if match_pool(&payload.whirlpool, &args.pools) {
                                    println!(
                                        "\t{} {:<44}  Initialized conffig: {}, a: {}{}, b: {}{}, ts: {}, price: {}, payer: {}",
                                        EVENT_ICON_INITIALIZED,
                                        payload.whirlpool,
                                        payload.config,
                                        payload.token_mint_a,
                                        get_token_icon(&payload.token_mint_a),
                                        payload.token_mint_b,
                                        get_token_icon(&payload.token_mint_b),
                                        payload.tick_spacing,
                                        payload.decimal_price,
                                        transaction.payer,
                                    );
                                }
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

fn match_pool(pool: &String, pools: &[String]) -> bool {
    if pools.is_empty() {
        return true;
    }
    pools.contains(pool)
}

fn get_token_icon(mint: &str) -> &str {
    match mint {
        "So11111111111111111111111111111111111111112" => TOKEN_ICON_SOL, // SOL
        "mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So" => TOKEN_ICON_XSOL, // mSOL
        "J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn" => TOKEN_ICON_XSOL, // jitoSOL
        "jupSoLaHXQiZZTSfEWMTRRgpnyFm8f6sZdosWBjx93v" => TOKEN_ICON_XSOL, // jupSOL
        "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v" => TOKEN_ICON_XUSD, // USDC
        "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB" => TOKEN_ICON_XUSD, // USDT
        "2b1kV6DkPAnxd5ixfnxCpjxmKwqjjaYmCZfHsFu24GXo" => TOKEN_ICON_XUSD, // PYUSD
        "3NZ9JMVBmGAqocybic2c7LQCJScmgsAZ6vQqTDzcqmJh" => TOKEN_ICON_XBTC, // WBTC
        "cbbtcf3aa214zXHbiAZQwf4122FBYbraNdFqgw4iMij" => TOKEN_ICON_XBTC, // cbBTC
        "7vfCXTUXx5WJV5JADk17DUJ4ksgau7utNKj4b963voxs" => TOKEN_ICON_XETH, // whETH
        _ => TOKEN_ICON_OTHER,
    }
}
