//! Data model for FCM notifications.
//!
//! This module defines the `Notification` struct and its builder for constructing notification
//! payloads within FCM messages. It includes fields for the title and body of the notification
//! and supports creating instances with a flexible and readable syntax.
use serde::{Deserialize, Serialize};

/// Represents a notification payload within an FCM message.
///
/// This struct is used to define the content of notifications sent through the Firebase Cloud Messaging (FCM) API.
/// It includes optional fields for the title and body text of the notification.
#[derive(Serialize, Deserialize, Debug)]
pub struct Notification {
    /// Title of the notification.
    pub title: Option<String>,
    /// Body text of the notification.
    pub body: Option<String>,
    // Add other notification fields (e.g., icon, click_action)
}

impl Notification {
    /// Creates a new `NotificationBuilder` instance for constructing `Notification` objects.
    ///
    /// # Returns
    ///
    /// Returns a `NotificationBuilder` that can be used to set fields and build a `Notification`.
    ///
    /// # Example
    ///
    /// ```
    /// use fcm_rs::notification::Notification;
    ///
    /// let notification = Notification::builder()
    ///     .title("Hello from Rust!")
    ///     .body("This is a test notification.")
    ///     .build();
    /// ```
    pub fn builder() -> NotificationBuilder {
        NotificationBuilder::default()
    }
}

/// Builder for constructing `Notification` objects.
///
/// The builder pattern allows you to create `Notification` instances with a flexible and
/// readable syntax, setting only the fields you need and leaving others as their default values.
#[derive(Default)]
pub struct NotificationBuilder {
    title: Option<String>,
    body: Option<String>,
}

impl NotificationBuilder {
    /// Sets the title of the notification.
    ///
    /// # Arguments
    ///
    /// * `title` - The title of the notification as a string slice.
    ///
    /// # Returns
    ///
    /// Returns the `NotificationBuilder` instance with the title set.
    ///
    /// # Example
    ///
    /// ```
    /// use fcm_rs::notification::Notification;
    ///
    /// let builder = Notification::builder().title("Hello from Rust!");
    /// ```
    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    /// Sets the body text of the notification.
    ///
    /// # Arguments
    ///
    /// * `body` - The body text of the notification as a string slice.
    ///
    /// # Returns
    ///
    /// Returns the `NotificationBuilder` instance with the body text set.
    ///
    /// # Example
    ///
    /// ```
    /// use fcm_rs::notification::Notification;
    ///
    /// let builder = Notification::builder().body("This is a test notification.");
    /// ```
    pub fn body(mut self, body: &str) -> Self {
        self.body = Some(body.to_string());
        self
    }

    /// Builds the `Notification` instance from the builder.
    ///
    /// # Returns
    ///
    /// Returns a `Notification` object with the fields set using the builder methods.
    ///
    /// # Example
    ///
    /// ```
    /// use fcm_rs::notification::Notification;
    ///
    /// let notification = Notification::builder()
    ///     .title("Hello from Rust!")
    ///     .body("This is a test notification.")
    ///     .build();
    /// ```
    pub fn build(self) -> Notification {
        Notification {
            title: self.title,
            body: self.body,
        }
    }
}
