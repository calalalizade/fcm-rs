use reqwest::Client;
use yup_oauth2::authenticator::Authenticator;
use yup_oauth2::hyper::client::HttpConnector;
use yup_oauth2::hyper_rustls::HttpsConnector;
use yup_oauth2::{ ServiceAccountAuthenticator, read_service_account_key };

use crate::error::FcmError;
use crate::models::{ FcmSendRequest, FcmSendResponse, Message };

pub struct FcmClient {
    auth: Authenticator<HttpsConnector<HttpConnector>>,
    http_client: Client,
    project_id: String, // Add project ID
}

impl FcmClient {
    pub async fn new(service_account_key_path: &str, project_id: String) -> Result<Self, FcmError> {
        let secret = read_service_account_key(service_account_key_path).await?;
        let auth = ServiceAccountAuthenticator::builder(secret).build().await?;
        Ok(Self { auth, http_client: Client::new(), project_id })
    }

    pub async fn send(&self, message: Message) -> Result<FcmSendResponse, FcmError> {
        let url = format!(
            "https://fcm.googleapis.com/v1/projects/{}/messages:send",
            self.project_id
        );

        let token = self.auth.token(&["https://www.googleapis.com/auth/firebase.messaging"]).await?;

        let request = FcmSendRequest {
            message,
            // Add other request parameters (e.g., validate_only) if needed
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
