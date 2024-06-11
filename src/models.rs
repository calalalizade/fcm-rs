use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub token: Option<String>,
    pub notification: Option<Notification>,
    pub data: Option<serde_json::Value>,
    // Add other FCM message fields as needed (e.g., condition, priority)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Notification {
    pub title: Option<String>,
    pub body: Option<String>,
    // Add other notification fields (e.g., icon, click_action)
}

#[derive(Serialize, Debug)]
pub struct FcmSendRequest {
    pub message: Message,
    // Add other request parameters (e.g., validate_only: bool) if needed
}

#[derive(Deserialize, Debug)]
pub struct FcmSendResponse {
    pub name: String, // Message ID
}
