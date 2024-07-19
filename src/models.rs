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

/// Represents the result of a sent FCM message.
#[derive(Deserialize)]
#[serde(untagged)]
pub enum FcmSendResult {
    /// A successful response from FCM.
    Success(FcmSuccessResponse),
    /// An error response from FCM.
    Error(FcmErrorResponse),
}

/// Represents a successful response from the FCM API after sending a message.
#[derive(Deserialize, Debug)]
pub struct FcmSuccessResponse {
    /// Message ID if the message was successfully processed
    pub name: String,
}

/// Represents an error response from the FCM API after sending a message.
#[derive(Serialize, Deserialize, Debug)]
pub struct FcmErrorResponse {
    /// Error if the message was unsuccessfully processed.
    pub error: ErrorResponse,
}

/// Contains the details of an error response from FCM.
/// Visit the [FCM Documentation](https://firebase.google.com/docs/cloud-messaging/send-message#rest) for details on the possible errors the API can respond with.
#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    /// The error code.
    pub code: usize,
    /// The error message.
    pub message: String,
    /// The error status
    pub status: String,
    /// Additional details about the error.
    pub details: Vec<Value>,
}
