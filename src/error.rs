//! Error types for FCM operations.
use crate::models::FcmErrorResponse;
use thiserror::Error;

/// Represents errors that can occur while using the FCM client.
#[derive(Error, Debug)]
pub enum FcmError {
    /// An error occurred during an HTTP request.
    #[error("Request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    /// An error occurred during authentication.
    #[error("Authentication failed: {0}")]
    AuthError(String),

    /// An error occurred during JSON serialization or deserialization.
    #[error("JSON serialization/deserialization failed: {0}")]
    JsonError(#[from] serde_json::Error),

    /// An error occurred related to OAuth2.
    #[error("OAuth2 error: {0}")]
    OAuth2Error(#[from] yup_oauth2::Error),

    /// An error occurred during file I/O operations.
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    // An error in the response from FCM.
    #[error("Response error: {0}")]
    ResponseError(FcmErrorResponse),
}
