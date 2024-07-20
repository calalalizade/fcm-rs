//! Provides the `FcmClient` struct for interacting with the FCM API.
use crate::error::FcmError;
use crate::models::{FcmSendRequest, FcmSendResult, FcmSuccessResponse, Message};
use reqwest::Client;
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
    /// Returns an `FcmError` if:
    ///
    /// * The service account key cannot be read.
    /// * The authenticator cannot be built.
    /// * Any other error occurs during initialization.
    pub async fn new(service_account_key_path: &str) -> Result<Self, FcmError> {
        let secret = read_service_account_key(service_account_key_path).await?;
        let project_id = match secret.project_id {
            Some(ref id) => id.clone(),
            None => {
                return Err(FcmError::AuthError(
                    "Service account key JSON file missing project ID".to_string(),
                ));
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
    /// This method constructs and sends an HTTP request to the FCM API to deliver a
    /// message to the specified recipients. It handles authentication, constructs the
    /// necessary request, and processes the response from the FCM service.
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
        let url = self.build_url();
        let token_str = self.get_token().await?;
        let request = FcmSendRequest { message };

        let response = self
            .http_client
            .post(url)
            .header("Authorization", format!("Bearer {:?}", token_str))
            .json(&request) // Send the request object
            .send()
            .await?;

        response
            .json::<FcmSendResult>()
            .await
            .map_err(FcmError::from)?
            .into()
    }

    /// Constructs the URL for sending FCM messages.
    ///
    /// # Returns
    ///
    /// Returns a `String` containing the URL for the FCM API endpoint.
    fn build_url(&self) -> String {
        format!(
            "https://fcm.googleapis.com/v1/projects/{}/messages:send",
            self.project_id
        )
    }

    /// Retrieves an OAuth2 token for the FCM API.
    ///
    /// This method requests an OAuth2 token from the authenticator that is required to authenticate
    /// requests to the FCM API.
    ///
    /// # Errors
    ///
    /// Returns an `FcmError` if:
    ///
    /// * The token cannot be retrieved from the authenticator.
    /// * Any other error occurs while fetching the token.
    ///
    /// # Returns
    ///
    /// On success, returns the OAuth2 token as a `String`. If the token cannot be retrieved,
    /// an `FcmError` is returned.
    async fn get_token(&self) -> Result<String, FcmError> {
        let token = self
            .auth
            .token(&["https://www.googleapis.com/auth/firebase.messaging"])
            .await?;

        token.token().map(|s| s.to_string()).ok_or_else(|| {
            FcmError::AuthError("Failed to retrieve token from authenticator".to_string())
        })
    }
}
