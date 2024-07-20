//! # FCM Rust Crate
//!
//! This crate provides a Rust client for the Firebase Cloud Messaging (FCM) API V1, allowing you to send notifications and data messages to Android and iOS devices.
//!
//! ## Features
//! * **FCM API V1 Support:** Utilizes the latest FCM API V1 endpoint for sending messages.
//! * **OAuth2 Authentication:**  Provides secure authentication using service account credentials.
//! * **Automatic Token Management:** Handles access token retrieval and refreshing seamlessly using `yup-oauth2`.
//! * **Error Handling:** Includes comprehensive error handling for API requests, authentication, and deserialization.

pub mod client;
pub mod error; // Custom error types for FCM interactions
pub mod models; // Data structures for FCM messages, responses, etc. // The FcmClient struct and methods to interact with FCM
