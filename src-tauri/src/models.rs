//! Data models for the application.

/// Short details about the user, for display in the UI.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct CustomUserDetails {
    /// The user ID.
    pub id: String,
    /// The email address of the user.
    pub email: String,
    /// Whether the user is suspended.
    pub suspended: bool,
    /// The organizations the user is a member of (just the names).
    pub organizations: Vec<String>,
}
