use crate::{ws, Clients, Result};
use warp::Reply;

// Accepts a HashMap (type Clients)
// Calls 'client_connection' function from 'ws.rs' and returns the result
// A Reply is a type that can be converted into an HTTP response to be sent to the client
// Authenication logic could go in here
pub async fn ws_handler(ws: warp::ws::Ws, clients: Clients) -> Result<impl Reply> {
    println!("ws_handler");

    Ok(ws.on_upgrade(move |socket| ws::client_connection(socket, clients)))
}
