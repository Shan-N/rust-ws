use std::{net::SocketAddr, sync::Arc};
use crate::channel::ChannelManager;
use tokio_tungstenite::{tungstenite::Message, accept_async};
use tokio::net::TcpStream;
use futures::{stream::StreamExt, SinkExt};
use serde::{Deserialize, Serialize};
use crate::db::init_db;

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatMessage {
    pub command: Option<String>,
    pub sender_id: Option<String>,
    pub receiver_id: Option<String>,
    pub contract_id: Option<String>,
    pub username: Option<String>,
    pub message: Option<String>,
}

pub async fn handle_connection(stream: TcpStream, addr: SocketAddr, channel_manager: Arc<ChannelManager>) {
    let ws_stream = accept_async(stream)
        .await
        .expect("Error during WebSocket handshake");

    let (mut sender, mut receiver) = ws_stream.split();
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    let channel_manager = channel_manager.clone();

    // Outgoing task
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sender.send(msg).await.is_err() {
                break;
            }
        }
    });

    let mut current_channel = String::new();


    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                if let Ok(chat_msg) = serde_json::from_str::<ChatMessage>(&text) {
                    match chat_msg.command.as_deref() {
                        Some("CREATE") | Some("JOIN") => {
                            if let Some(room_name) = chat_msg.contract_id.clone() {
                                channel_manager.add_or_create(room_name.clone()).await;
                                channel_manager.add_sender(room_name.clone(), tx.clone()).await;
                                channel_manager
                                    .broadcast(
                                        room_name.clone(),
                                        Message::Text(format!("User {} joined the channel", chat_msg.username.unwrap_or_default()).into()),
                                    )
                                    .await;
                                current_channel = room_name;
                            }
                        }
                        Some("LEAVE") => {
                            if !current_channel.is_empty() {
                                channel_manager
                                    .remove_sender(current_channel.clone(), &tx)
                                    .await;
                                channel_manager
                                    .broadcast(
                                        current_channel.clone(),
                                        Message::Text(format!("User {} left the channel", addr).into()),
                                    )
                                    .await;
                                current_channel.clear();
                            }
                        }
                        Some("MESSAGE") => {
                            if let (Some(channel), Some(username), Some(content)) = (
                                chat_msg.contract_id.clone(),
                                chat_msg.username.clone(),
                                chat_msg.message.clone(),
                            ) {
                                channel_manager
                                    .broadcast(
                                        channel,
                                        Message::Text(format!("{}: {}", username, content).into()),
                                    )
                                    .await;
                            }
                            if let Err(e) = init_db(&chat_msg).await {
                                eprintln!("Failed to save message to database: {}", e);
                            }
                        }
                        _ => {
                            if tx.send(Message::Text("Unknown command".into())).is_err() {
                                eprintln!("Failed to send message");
                            }
                        }
                    }
                }
            }
            Ok(Message::Close(_)) => {
                break;
            }
            Ok(_) => {} 
            Err(e) => {
                eprintln!("Error processing message from {}: {}", addr, e);
                break;
            }
        }
    }
}
