use reqwest::Client;
use std::error::Error;
use std::collections::HashMap;

async fn send_pushover_notification(title: &str, message: &str, priority: i32) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let mut params = HashMap::new();
    params.insert("token".to_string(), "token_key".to_string());
    params.insert("user".to_string(), "user_key".to_string());
    params.insert("message".to_string(), message.to_string());
    params.insert("title".to_string(), title.to_string());
    params.insert("priority".to_string(), priority.to_string());

    // Add expire and retry parameters if priority is 2
    if priority == 2 {
        params.insert("expire".to_string(), "3600".to_string()); // 1 hour expiration time
        params.insert("retry".to_string(), "30".to_string());   // Retry every 60 seconds
    }

    let _res = client
        .post("https://api.pushover.net/1/messages.json")
        .form(&params)
        .send()
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Run a test
    send_pushover_notification("Test from Rust", &format!("Very High!"), 2).await?;
    send_pushover_notification("Test from Rust", &format!("High!"), 1).await?;
    send_pushover_notification("Test from Rust", &format!("Normal!"), 0).await?;

    Ok(())
}
