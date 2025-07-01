use tokio::sync::{mpsc, Mutex};
use tokio_tungstenite::tungstenite::Message;
use std::{collections::HashMap, sync::Arc};

pub struct ChannelManager {
    pub channels: Arc<Mutex<HashMap<String, Vec<mpsc::UnboundedSender<Message>>>>>,
}

impl ChannelManager {
    pub fn new() -> Self {
        ChannelManager { channels: Arc::new(Mutex::new(HashMap::new())) }
    }
    pub async fn add_or_create(&self, channel_name:String){
        let mut channels = self.channels.lock().await;
        if !channels.contains_key(&channel_name) {
            channels.insert(channel_name.clone(), Vec::new());
        }
    }
    pub async fn add_sender(&self, channel_name: String, sender: mpsc::UnboundedSender<Message>){
        let mut channels = self.channels.lock().await;
        if let Some(senders) = channels.get_mut(&channel_name) {
            senders.push(sender);
        }
    }
    pub async fn remove_sender(&self, channel_name: String, sender: &mpsc::UnboundedSender<Message>) {
        let mut channels = self.channels.lock().await;
        if let Some(senders) = channels.get_mut(&channel_name) {
            senders.retain(|s| s as *const _ != sender as *const _);
            if senders.is_empty() {
                channels.remove(&channel_name);
            }
        }
    }
    pub async fn broadcast(&self, channel_name: String, message: Message){
        let channels = self.channels.lock().await;
        if let Some(senders) = channels.get(&channel_name){
            for sender in senders.iter() {
                sender.send(message.clone()).expect("Failed to send message");
            }
        }
    }
}