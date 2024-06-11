use fcm_rust::{ client::FcmClient, models::{ Message, Notification } };

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ... (load service account key path and project ID)

    let client = FcmClient::new("./service_account.json", "login-80680".to_string()).await?;

    let message = Message {
        token: Some("your_device_token".to_string()),
        notification: Some(Notification {
            title: Some("Hello from Rust!".to_string()),
            body: Some("This is a test notification.".to_string()),
        }),
        data: None,
    };

    let response = client.send(message).await?;
    println!("FCM response: {:?}", response);

    Ok(())
}
