# warp-websocket
Rust WebSocket connection to the binance WebSocket

Stream the bid / ask prices and size for luna-usdt and luna-btc trading pairs.

Run the server with `cargo run` from the command line

To connect a client:

Use an app like WebSocket Client (from the App store) or websocat (from the command line) to connect to the stream via 127.0.0.1:9999/luna

