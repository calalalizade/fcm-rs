//! Data models for FCM requests.
use crate::models::Message;
use serde::Serialize;

/// Represents a request to send an FCM message.
#[derive(Serialize, Debug)]
pub struct FcmSendRequest {
    /// The FCM message to send.
    pub message: Message,
    // Add other request parameters (e.g., validate_only: bool) if needed
}
