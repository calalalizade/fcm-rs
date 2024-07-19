//! Provides the `FcmClient` struct for interacting with the FCM API.
use crate::error::FcmError;
use crate::models::{FcmErrorResponse, FcmSendRequest, FcmSendResult, FcmSuccessResponse, Message};
use reqwest::Client;
use serde_json::json;
use std::fmt::Display;
use yup_oauth2::authenticator::Authenticator;
use yup_oauth2::hyper::client::HttpConnector;
use yup_oauth2::hyper_rustls::HttpsConnector;
use yup_oauth2::{read_service_account_key, ServiceAccountAuthenticator};

/// Firebase Cloud Messaging (FCM) client.
pub struct FcmClient {
    /// Authenticator for OAuth2.
    auth: Authenticator<HttpsConnector<HttpConnector>>,
    /// HTTP client for making requests.
    http_client: Client,
    /// Firebase project ID.
    project_id: String,
}

impl FcmClient {
    /// Creates a new `FcmClient` instance.
    ///
    /// # Arguments
    ///
    /// * `service_account_key_path` - Path to the service account key JSON file.
    ///
    /// # Errors
    ///
    /// Returns an `FcmError` if the service account key cannot be read,
    /// the authenticator cannot be built, or any other error occurs during initialization.
    pub async fn new(service_account_key_path: &str) -> Result<Self, FcmError> {
        let secret = read_service_account_key(service_account_key_path).await?;
        let project_id = match secret.project_id {
            Some(ref id) => id.clone(),
            None => {
                return Err(FcmError::AuthError(
                    "Service account key JSON file missing project ID".to_string(),
                ))
            }
        };
        let auth = ServiceAccountAuthenticator::builder(secret).build().await?;
        Ok(Self {
            auth,
            http_client: Client::new(),
            project_id,
        })
    }

    /// Sends an FCM message.
    ///
    /// # Arguments
    ///
    /// * `message` - The `Message` to send.
    ///
    /// # Errors
    ///
    /// Returns an `FcmError` if there's an issue with the request, authentication,
    /// JSON (de)serialization, the response, or any other error during the sending process.
    pub async fn send(&self, message: Message) -> Result<FcmSuccessResponse, FcmError> {
        let url = format!(
            "https://fcm.googleapis.com/v1/projects/{}/messages:send",
            self.project_id
        );

        let token = self
            .auth
            .token(&["https://www.googleapis.com/auth/firebase.messaging"])
            .await?;

        let request = FcmSendRequest { message };

        let response = self
            .http_client
            .post(url)
            .header(
                "Authorization",
                format!("Bearer {:?}", token.token().unwrap()),
            )
            .json(&request) // Send the request object
            .send()
            .await?;

        response
            .json::<FcmSendResult>()
            .await
            .map_err(FcmError::from)?
            .into()
    }
}

impl Into<Result<FcmSuccessResponse, FcmError>> for FcmSendResult {
    fn into(self) -> Result<FcmSuccessResponse, FcmError> {
        match self {
            FcmSendResult::Success(success) => Ok(success),
            FcmSendResult::Error(error) => Err(FcmError::ResponseError(error)),
        }
    }
}

impl Display for FcmErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", json!(&self))
    }
}
