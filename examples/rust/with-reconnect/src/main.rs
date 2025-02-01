use whirlpool_stream_websocket_client::{
    AccountParam, EventParam, WhirlpoolStreamMessage, WhirlpoolStreamWebsocketClient,
};

async fn read_stream_with_reconnect(
    endpoints: [&str; 2],
    apikey: &str,
    slot: Option<u64>,
    event_param: EventParam,
    account_param: AccountParam,
) {
    let mut endpoint_index = 0;
    let mut start_slot = slot;

    loop {
        let endpoint = endpoints[endpoint_index];
        endpoint_index = (endpoint_index + 1) % endpoints.len();

        println!(
            "Connecting to endpoint: {} slot: {:?}",
            endpoint, start_slot
        );

        if let Ok(mut client) = WhirlpoolStreamWebsocketClient::connect(
            endpoint,
            apikey,
            start_slot,
            Some(u32::MAX),
            event_param,
            account_param,
        )
        .await
        {
            while let Some(message) = client.next().await {
                match message {
                    Ok(WhirlpoolStreamMessage::Data {
                        slot,
                        block_height,
                        block_time,
                        events,
                        accounts,
                    }) => {
                        let now = std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_secs() as i64;
                        let time_diff = now - block_time;

                        start_slot = Some(slot);

                        println!(
                            "Data: slot:{}, height:{}, time:{} ({}s ago), events:{}, accounts:{}",
                            slot,
                            block_height,
                            block_time,
                            time_diff,
                            events.len(),
                            accounts.len()
                        );
                    }
                    Ok(WhirlpoolStreamMessage::Heartbeat) => println!("Heartbeat"),
                    Ok(WhirlpoolStreamMessage::Closed { reason }) => println!("Closed: {}", reason),
                    Err(e) => eprintln!("ERROR: {}", e),
                }
            }

            println!("Connection closed");
        } else {
            println!("Connection failed");
        }

        // reconnect wait
        println!("Reconnect wait...");
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}

#[tokio::main]
async fn main() {
    let endpoint_a = "wss://orcanauts-a.whirlpool-stream.pleiades.dev";
    let endpoint_b = "wss://orcanauts-b.whirlpool-stream.pleiades.dev";
    let apikey = "demo";

    read_stream_with_reconnect(
        [endpoint_a, endpoint_b],
        apikey,
        None,
        EventParam::All,
        AccountParam::All,
    )
    .await;
}
