//! Data models for FCM messages, requests, and responses.
use serde::{Deserialize, Serialize};
use serde_json::Value;

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

/// Represents a notification payload within an FCM message.
#[derive(Serialize, Deserialize, Debug)]
pub struct Notification {
    /// Title of the notification.
    pub title: Option<String>,
    /// Body text of the notification.
    pub body: Option<String>,
    // Add other notification fields (e.g., icon, click_action)
}

/// Represents a request to send an FCM message.
#[derive(Serialize, Debug)]
pub struct FcmSendRequest {
    /// The FCM message to send.
    pub message: Message,
    // Add other request parameters (e.g., validate_only: bool) if needed
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum FcmSendResult {
    Success(FcmSuccessResponse),
    Error(FcmErrorResponse),
}

/// Represents a response from the FCM API after sending a message.
#[derive(Deserialize, Debug)]
pub struct FcmSuccessResponse {
    /// Message ID if the message was successfully processed
    pub name: String,
}

/// Represents an error from the FCM API after sending a message.
#[derive(Serialize, Deserialize, Debug)]
pub struct FcmErrorResponse {
    /// Error if the message was unsuccessfully processed
    pub error: ErrorResponse,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub code: usize,
    pub message: String,
    pub status: String,
    pub details: Vec<Value>,
}
