//! Provides the `FcmClient` struct for interacting with the FCM API.
use reqwest::Client;
use yup_oauth2::authenticator::Authenticator;
use yup_oauth2::hyper::client::HttpConnector;
use yup_oauth2::hyper_rustls::HttpsConnector;
use yup_oauth2::{ ServiceAccountAuthenticator, read_service_account_key };

use crate::error::FcmError;
use crate::models::{ FcmSendRequest, FcmSendResponse, Message };

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
    /// * `project_id` - Firebase project ID.
    ///
    /// # Errors
    ///
    /// Returns an `FcmError` if the service account key cannot be read,
    /// the authenticator cannot be built, or any other error occurs during initialization.
    pub async fn new(service_account_key_path: &str, project_id: String) -> Result<Self, FcmError> {
        let secret = read_service_account_key(service_account_key_path).await?;
        let auth = ServiceAccountAuthenticator::builder(secret).build().await?;
        Ok(Self { auth, http_client: Client::new(), project_id })
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
    /// JSON (de)serialization, or any other error during the sending process.
    pub async fn send(&self, message: Message) -> Result<FcmSendResponse, FcmError> {
        let url = format!(
            "https://fcm.googleapis.com/v1/projects/{}/messages:send",
            self.project_id
        );

        let token = self.auth.token(&["https://www.googleapis.com/auth/firebase.messaging"]).await?;

        let request = FcmSendRequest {
            message,
        };

        let response = self.http_client
            .post(url)
            .header("Authorization", format!("Bearer {:?}", token.token().unwrap()))
            .json(&request) // Send the request object
            .send().await?;

        // let response_text = response.text().await?; // Get raw response
        // println!("Raw FCM response: {}", response_text); // Print for debugging

        response.json::<FcmSendResponse>().await.map_err(FcmError::from)

        // let parsed_response: Result<FcmSendResponse, _> = serde_json::from_str(&response_text);
        // parsed_response.map_err(FcmError::from)
    }
}
