//! A Tauri app to provide a fast and responsive UI for changing cloudflare DNS.
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use cloudflare::{
    BearerAuthorizer, CloudflareAuthorizer, CloudflareListZonesResponse, CloudflareResponse,
    CloudflareUserDetailsResponse,
};

mod cloudflare;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn get_zones(
    token: &str,
) -> Result<CloudflareResponse<Vec<CloudflareListZonesResponse>>, ()> {
    let authorizer = cloudflare::BearerAuthorizer {
        token: token.to_string(),
    };
    let client = reqwest::Client::new();

    let request_builder = client
        .get("https://api.cloudflare.com/client/v4/zones")
        .header("Content-Type", "application/json");

    let request_builder = authorizer.with_auth(request_builder);

    let response: CloudflareResponse<Vec<CloudflareListZonesResponse>> = request_builder
        .send()
        .await
        .map_err(|e| {
            tracing::error!("Failed to send request");
            tracing::error!("{:?}", e);
        })?
        .json()
        .await
        .map_err(|e| {
            tracing::error!("Failed to parse response as JSON");
            tracing::error!("{:?}", e);
        })?;

    Ok(response) // Return the response to the frontend
}

/// Check if the API key is valid, by making a request to the Cloudflare API.
///
/// We request to User Details and then pull a couple of fields from the response:
///
/// - `id`: The user ID.
/// - `email`: The email address of the user.
/// - `suspended`: Whether the user is suspended.
/// - `organizations`: The organizations the user is a member of (just the names).
#[tauri::command]
async fn check_api_key(token: &str) -> Result<CustomUserDetails, ()> {
    let authorizer = BearerAuthorizer {
        token: token.to_string(),
    };
    let client = reqwest::Client::new();

    let request_builder = client
        .get("https://api.cloudflare.com/client/v4/user")
        .header("Content-Type", "application/json");

    let request_builder = authorizer.with_auth(request_builder);

    let response: CloudflareResponse<CloudflareUserDetailsResponse> = request_builder
        .send()
        .await
        .map_err(|e| {
            tracing::error!("Failed to send request");
            tracing::error!("{:?}", e);
        })?
        .json()
        .await
        .map_err(|e| {
            tracing::error!("Failed to parse response as JSON");
            tracing::error!("{:?}", e);
        })?;

    Ok(CustomUserDetails {
        id: response.result.id.clone(),
        email: response.result.email.clone(),
        suspended: response.result.suspended,
        organizations: response
            .result
            .organizations
            .iter()
            .map(|org| org.name.clone())
            .collect(),
    }) // Return the response to the frontend
}

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

fn main() {
    tracing_subscriber::fmt::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_zones, check_api_key])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
