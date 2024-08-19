# Firebase Cloud Messaging API v1 Rust Crate

This Rust crate provides a convenient way to send notifications using Firebase Cloud Messaging (FCM) API v1. It leverages async/await for asynchronous operations and supports loading service account credentials from a JSON file.

## Installation

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
fcm-rs = "0.3.0"
```

## Usage

### Basic Usage

Below is an example of how to use this crate to send a notification using the traditional method:

```rust
use fcm_rs::{client::FcmClient, models::{Message, Notification}};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service_account_path = "path/to/service_account";

    // Create a new FCM client
    let client = FcmClient::new(service_account_path).await?;

    // Define the message with the target device token and notification details
    let message = Message {
        token: Some("your_device_token".to_string()),
        notification: Some(Notification {
            title: Some("Hello from Rust!".to_string()),
            body: Some("This is a test notification.".to_string()),
        }),
        data: None,
    };

    // Send the message and handle the response
    let response = client.send(message).await?;
    println!("FCM response: {:?}", response);

    Ok(())
}
```

### Builder Pattern Usage

For a more flexible and readable way to create messages and notifications, you can use the builder pattern. Here’s an example:

```rust
use fcm_rs::{client::FcmClient, models::{Message, Notification}};

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
```

In this example, `Message::builder()` and `Notification::builder()` provide a more fluent and flexible way to construct instances, allowing you to set only the fields you need and improve code readability.

## Contribution

Contributions are welcome! Feel free to submit issues or pull requests.

## Contact

For any queries or suggestions, please open an issue on the GitHub repository.
