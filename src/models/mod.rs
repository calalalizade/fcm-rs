//! ## Modules
//!
//! - **`message`**: Defines the `Message` struct and its builder for constructing FCM messages.
//! - **`notification`**: Defines the `Notification` struct and its builder for constructing notification payloads.
//! - **`request`**: Defines the `FcmSendRequest` struct used for sending requests to the FCM API.
//! - **`response`**: Defines the response types from the FCM API, including `FcmSendResult`, `FcmSuccessResponse`, and `FcmErrorResponse`.
pub mod message; // Defines the `Message` struct and `MessageBuilder` for constructing messages.
pub mod notification; // Defines the `Notification` struct and `NotificationBuilder` for constructing notifications.
pub mod request; // Defines the `FcmSendRequest` struct for sending requests to the FCM API.
pub mod response; // Defines response types from the FCM API, including `FcmSendResult`, `FcmSuccessResponse`, and `FcmErrorResponse`.

pub use message::Message; // Re-exports `Message` for easy access.
pub use notification::Notification; // Re-exports `Notification` for easy access.
pub use request::FcmSendRequest; // Re-exports `FcmSendRequest` for easy access.
pub use response::{FcmErrorResponse, FcmSendResult, FcmSuccessResponse}; // Re-exports response types for easy access.
