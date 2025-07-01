use std::sync::Arc;
use crate::websocket::handle_connection;
use tokio::net::TcpListener;

use crate::channel::ChannelManager;


pub async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = TcpListener::bind("127.0.0.1:8080").await?;
     println!("webSocket server Started at ws://127.0.0.1:8080");
    
    let channel_manager = Arc::new(ChannelManager::new());

    while let Ok((stream, addr)) = addr.accept().await {
        let channel_manager: Arc<ChannelManager> = channel_manager.clone();
        tokio::spawn(handle_connection(stream, addr, channel_manager));
    }

    Ok(())
}