use thiserror::Error;

#[derive(Error, Debug)]
pub enum FcmError {
    #[error("Request failed: {0}")] RequestError(#[from] reqwest::Error),
    #[error("Authentication failed: {0}")] AuthError(String),
    #[error("JSON serialization/deserialization failed: {0}")] JsonError(#[from] serde_json::Error),
    #[error("OAuth2 error: {0}")] OAuth2Error(#[from] yup_oauth2::Error),
    #[error("IO error: {0}")] // <-- New variant for std::io::Error
    IoError(#[from] std::io::Error),
}
