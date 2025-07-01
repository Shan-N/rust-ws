mod server;
mod channel;
mod websocket;
mod db;

#[tokio::main]
async fn main() {
    if let Err(e) = server::start_server().await {
        eprintln!("Error starting server: {}", e);
    }
}
