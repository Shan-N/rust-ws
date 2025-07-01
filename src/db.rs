use dotenvy::dotenv;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use std::env;
use crate::websocket::ChatMessage;

pub async fn init_db(message: &ChatMessage) -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let supabase_url = env::var("NEXT_PUBLIC_SUPABASE_URL").expect("NEXT_PUBLIC_SUPABASE_URL not set");
    let supabase_anon_key = env::var("NEXT_PUBLIC_SUPABASE_ANON_KEY").expect("NEXT_PUBLIC_SUPABASE_ANON_KEY not set");
    
    let client = reqwest::Client::new();

    let payload = serde_json::json!({
        "sender_id": message.sender_id,
        "recipient_id": message.receiver_id,
        "contract_id": message.contract_id,
        "message": message.message,
    });

    let res = client.post(format!("{}/rest/v1/messages", supabase_url))
        .header(AUTHORIZATION, format!("Bearer {}", supabase_anon_key))
        .header("apikey", supabase_anon_key)
        .header(CONTENT_TYPE, "application/json")
        .header("Prefer", "return=minimal")
        .json(&payload)
        .send()
        .await?;

    if res.status().is_success() {
        println!("Message saved successfully");
    } else {
        eprintln!("Failed to save message: {}", res.status());
    }

    Ok(())
}