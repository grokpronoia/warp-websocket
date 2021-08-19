use std::{collections::HashMap, convert::Infallible, sync::Arc};
use tokio::sync::{mpsc, Mutex};
use url::Url;
use warp::{ws::Message, Filter, Rejection};

mod handlers;
mod models;
mod workers;
mod ws;

static BINANCE_WS_API: &str = "wss://stream.binance.com:9443";

// Represents the connected Client
// 'mpsc::UnboundedSender<>' sends messages without blocking program to our UnboundedReceiver client
#[derive(Debug, Clone)]
pub struct Client {
      pub client_id: String,
      pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

// Set 'type' alias to keep track of connected clients
// Mutex -> locks the resource to prevent race conditions
// Arc -> thread safe reference counter
type Clients = Arc<Mutex<HashMap<String, Client>>>;

// Set 'type' alias to make code easier to write and read
type Result<T> = std::result::Result<T, Rejection>;

fn get_binance_streams_url() -> Url {
    let binance_url = format!(
        "{}/stream?streams=lunausdt@depth5@1000ms/lunabtc@depth5@1000ms",
        BINANCE_WS_API
    );

    Url::parse(&binance_url).unwrap()
}

// Set main function to be 'async' with keyword and tokio macro
#[tokio::main]
async fn main () {
    // Create a new Client instance
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

    println!("Configuring websocket route.");

    // Create WebSocket route for our server
    // 'warp::ws())' ->  upgrade web socket
    // Pass in ‘with_clients’ function defined below
    // Pass in 'ws_handler' function from 'handlers.rs'
    let ws_route = warp::path("luna")
	.and(warp::ws())
	.and(with_clients(clients.clone()))
	.and_then(handlers::ws_handler);

    // 'warp::cors()' (Cross Origin Resource Sharing) -> allows requests for anyone
    let routes = ws_route.with(warp::cors().allow_any_origin());

    println!("Connecting to binance stream...");
 
    // Connect to Binance server    
    let binance_url = get_binance_streams_url();
    let (socket, response) = tungstenite::connect(binance_url).expect("Can't connect.");
    println!("Connected to binance stream.");
    println!("HTTP status code: {}", response.status());
    println!("Response headers:");
    for (ref header, ref header_value) in response.headers() {
        println!("- {}: {:?}", header, header_value);
    }

    println!("Starting update loop");
    
    // Call async 'main_worker' function from 'workers.rs'
    // Track the number of connected clients from 'clients'
    // 'tokio::task::spawn' creates a new asynchronous task and we 'move' the variable into closure
    // Pass the socket connection to 'main_worker' function in 'workers.rs'
    tokio::task::spawn(async move {
        workers::main_worker(clients.clone(), socket).await;
    });

    println!("Starting server.");
    // Start the server on 127.0.0.1:9999/luna
    warp::serve(routes).run(([127, 0, 0, 1], 9999)).await;
}

// Accepts a new thread-safe, reference-counting hash map
// Extract the data and returns a clone of the argument passed (we set Error = Infallible bc it can not fail)
fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}
