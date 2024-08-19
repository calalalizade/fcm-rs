use fcm_rs::{
    client::FcmClient,
    models::{Message, Notification},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service_account_path = "./service_account.json";

    // Create a new FCM client
    let client = FcmClient::new(service_account_path).await?;

    // Build the notification
    let notification = Notification::builder()
        .title("Hello from Rust!")
        .body("This is a test notification.")
        .build();

    // Build the message
    let message = Message::builder()
        .token("your_device_token")
        .notification(notification)
        .build();

    // Send the message and handle the response
    let response = client.send(message).await?;
    println!("FCM response: {:?}", response);

    Ok(())
}
