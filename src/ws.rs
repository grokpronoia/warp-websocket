use crate::{Client, Clients};
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;
use warp::ws::{Message, WebSocket};

// Creates an instance of the 'Client' struct and adds it to the 'clients' HashMap
// Receives a ping / Sends a pong
pub async fn client_connection(ws: WebSocket, clients: Clients) {
    println!("establishing client connection... {:?}", ws);

    // Split WebSocket stream into sender / receiver
    // Sender used to establish connection to UnboundedChannel
    // Receiver used to receive messages from client - needs to be mut bc mssgs will change
    let (client_ws_sender, mut client_ws_rcv) = ws.split();

    // Create the UnboundedChannel to send messages to the client
    let (client_sender, client_rcv) = mpsc::unbounded_channel();

    // Receives a stream of values from the UnboundedSender
    let client_rcv = UnboundedReceiverStream::new(client_rcv);
    
    // Keeps the UnboundedChannel connection open until client disconnects
    tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
        if let Err(e) = result {
            println!("error sending websocket msg: {}", e);
        }
    }));

    let uuid = Uuid::new_v4().to_simple().to_string();
    
    // Instantiate the Client struct
    // Use 'client_sender' in 'new_client' 'Client' so we can send messages to the connected client
    let new_client = Client {
        client_id: uuid.clone(),
        sender: Some(client_sender),
    };

    // Add the 'Client' to the 'clients' HashMap
    clients.lock().await.insert(uuid.clone(), new_client);

    // Create a loop to receive the stream of messages from client
    // Break the loop when the client disconnects
    while let Some(result) = client_ws_rcv.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                println!("error receiving message for id {}): {}", uuid.clone(), e);
                break;
            }
        };    
    }
 
    // Break the loop when client disconnects
    // Remove the client ID from clients HashMap
    clients.lock().await.remove(&uuid);
    println!("{} disconnected", uuid);
}
