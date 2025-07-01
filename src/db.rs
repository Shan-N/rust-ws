use dotenvy::dotenv;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use std::env;
use crate::websocket::ChatMessage;

pub async fn init_db(message: &ChatMessage) -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let supabase_url = env::var("NEXT_PUBLIC_SUPABASE_URL").expect("NEXT_PUBLIC_SUPABASE_URL not set");
    let supabase_key = env::var("SUPABASE_KEY").expect("SUPABASE_KEY not set");
    
    let client = reqwest::Client::new();

    let payload = serde_json::json!({
        "sender_id": message.sender_id.as_ref().unwrap(),
        "recipient_id": message.receiver_id.as_ref().unwrap(),
        "contract_id": message.contract_id.as_ref().unwrap(),
        "message": message.message.as_ref().unwrap(),
    });

    let res = client.post(format!("{}/rest/v1/messages", supabase_url))
        .header(AUTHORIZATION, format!("Bearer {}", supabase_key))
        .header("apikey", supabase_key)
        .header(CONTENT_TYPE, "application/json")
        .header("Prefer", "return=minimal")
        .json(&payload)
        .send()
        .await?;

        if res.status().is_success() {
            println!("✅ Message saved successfully");
        } else {
            let body = res.text().await?;
            eprintln!("Response Body: {}", body);
        }


    Ok(())
}