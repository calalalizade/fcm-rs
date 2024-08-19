//! Data models for FCM messages.
//!
//! This module defines the `Message` struct and its builder for constructing FCM messages
//! to be sent through the Firebase Cloud Messaging (FCM) API. It includes fields for the
//! registration token or topic, notification payload, and custom data payload.
use super::notification::Notification;
use serde::{Deserialize, Serialize};

/// Represents an FCM message to be sent.
#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    /// Registration token of the target device or the topic name for subscription.
    pub token: Option<String>,
    /// Notification payload.
    pub notification: Option<Notification>,
    /// Custom data payload.
    pub data: Option<serde_json::Value>,
    // Add other FCM message fields as needed (e.g., condition, priority)
}

impl Message {
    /// Creates a new `MessageBuilder` instance for constructing `Message` objects.
    ///
    /// # Returns
    ///
    /// Returns a `MessageBuilder` that can be used to set fields and build a `Message`.
    ///
    /// # Example
    ///
    /// ```
    /// use fcm_rs::models::{Message, Notification};
    ///
    /// let message = Message::builder()
    ///     .token("your_device_token")
    ///     .notification(
    ///         Notification::builder()
    ///             .title("Hello from Rust!")
    ///             .body("This is a test notification.")
    ///             .build(),
    ///     )
    ///     .build();
    /// ```
    pub fn builder() -> MessageBuilder {
        MessageBuilder::default()
    }
}

/// Builder for constructing `Message` objects.
///
/// The builder pattern allows you to create `Message` instances with a flexible and
/// readable syntax, setting only the fields you need and leaving others as their default values.
#[derive(Default)]
pub struct MessageBuilder {
    token: Option<String>,
    notification: Option<Notification>,
    data: Option<serde_json::Value>,
}

impl MessageBuilder {
    /// Sets the registration token or topic name for the message.
    ///
    /// # Arguments
    ///
    /// * `token` - The registration token or topic name as a string slice.
    ///
    /// # Returns
    ///
    /// Returns the `MessageBuilder` instance with the token set.
    ///
    /// # Example
    ///
    /// ```
    /// use fcm_rs::models::Message;
    ///
    /// let builder = Message::builder().token("your_device_token");
    /// ```
    pub fn token(mut self, token: &str) -> Self {
        self.token = Some(token.to_string());
        self
    }

    /// Sets the notification payload for the message.
    ///
    /// # Arguments
    ///
    /// * `notification` - The `Notification` object containing title and body of the notification.
    ///
    /// # Returns
    ///
    /// Returns the `MessageBuilder` instance with the notification payload set.
    ///
    /// # Example
    ///
    /// ```
    /// use fcm_rs::models::{Message, Notification};
    ///
    /// let notification = Notification::builder()
    ///     .title("Hello from Rust!")
    ///     .body("This is a test notification.")
    ///     .build();
    ///
    /// let builder = Message::builder().notification(notification);
    /// ```
    pub fn notification(mut self, notification: Notification) -> Self {
        self.notification = Some(notification);
        self
    }

    /// Sets the custom data payload for the message.
    ///
    /// # Arguments
    ///
    /// * `data` - The custom data payload as a `serde_json::Value`.
    ///
    /// # Returns
    ///
    /// Returns the `MessageBuilder` instance with the custom data payload set.
    ///
    /// # Example
    ///
    /// ```
    /// use fcm_rs::models::{Message};
    /// use serde_json::json;
    ///
    /// let data = json!({
    ///     "key": "value"
    /// });
    ///
    /// let builder = Message::builder().data(data);
    /// ```
    pub fn data(mut self, data: serde_json::Value) -> Self {
        self.data = Some(data);
        self
    }

    /// Builds the `Message` instance from the builder.
    ///
    /// # Returns
    ///
    /// Returns a `Message` object with the fields set using the builder methods.
    pub fn build(self) -> Message {
        Message {
            token: self.token,
            notification: self.notification,
            data: self.data,
        }
    }
}
