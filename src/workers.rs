use crate::{models, Clients};
use tokio::time::Duration;
use tungstenite::{client::AutoStream, WebSocket};
use warp::ws::Message;

// Async function with infinite loop
pub async fn main_worker(clients: Clients, mut socket: WebSocket<AutoStream>) {
    loop {
	// Lock clients and Track the number of connected clients
        tokio::time::sleep(Duration::from_millis(2000)).await;
        let connected_client_count = clients.lock().await.len();
        if connected_client_count == 0 {
            println!("No clients connected, skip sending data");
            continue;
        }
        println!("{} connected client(s)", connected_client_count);

    	// Receive data from binance socket in 'main.rs'
        let msg = socket.read_message().expect("Error reading message");
        let msg = match msg {
            tungstenite::Message::Text(s) => s,
            _ => {
                panic!("Error getting text");
            }
        };

  	// Parse the binance data into our 'DepthStreamWrapper' struct from 'models.rs'
        let parsed: models::DepthStreamWrapper = serde_json::from_str(&msg).expect("Can't parse");
        for i in 0..parsed.data.asks.len() {
            println!(
                "{}: {}. ask: {}, size: {}",
                parsed.stream, i, parsed.data.asks[i].price, parsed.data.asks[i].size
            );
        }
	
	// Send the parsed data back to the client
	clients.lock().await.iter().for_each(|(_, client)| {
	    if let Some(sender) = &client.sender {
		let _ = sender.send(Ok(Message::text(serde_json::to_string(&parsed.data.asks).unwrap())));
	    }
	});
    }
}
