//! This module contains the Tauri commands that are exposed to the frontend.

use crate::{
    cloudflare::{
        BearerAuthorizer, CloudflareAuthorizer, CloudflareListZonesResponse, CloudflareResponse,
        CloudflareUserDetailsResponse, DNSRecord,
    },
    models::CustomUserDetails,
};

/// The base URL for the Cloudflare API.
pub const CLOUDFLARE_API_BASE: &str = "https://api.cloudflare.com/client/v4";

/// Get a list of zones the user has access to.
///
/// This command requires a token to be passed in, which is used to authenticate with the Cloudflare API.
///
/// # Errors
///
/// If the request fails, this function will return `Err(())`.
pub async fn get_zones(
    token: &str,
) -> Result<CloudflareResponse<Vec<CloudflareListZonesResponse>>, ()> {
    let authorizer = BearerAuthorizer {
        token: token.to_string(),
    };
    let client = reqwest::Client::new();

    let request_builder = client
        .get(format!("{CLOUDFLARE_API_BASE}/zones"))
        .header("Content-Type", "application/json");

    let request_builder = authorizer.with_auth(request_builder);

    tracing::info!("get_zones: Sending request to Cloudflare API");

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
///
/// # Errors
///
/// If the request fails, this function will return `Err(())`.
pub async fn check_api_key(token: &str) -> Result<CustomUserDetails, ()> {
    let authorizer = BearerAuthorizer {
        token: token.to_string(),
    };
    let client = reqwest::Client::new();

    let request_builder = client
        .get(format!("{CLOUDFLARE_API_BASE}/user"))
        .header("Content-Type", "application/json");

    let request_builder = authorizer.with_auth(request_builder);

    tracing::info!("check_api_key: Sending request to Cloudflare API");

    let response = request_builder.send().await.map_err(|e| {
        tracing::error!("Failed to send request");
        tracing::error!("{:?}", e);
    })?;

    if response.status() != 200 {
        tracing::error!("Failed to get user details");
        tracing::error!("{response:?}");
        tracing::error!("Provided token: {token}");
        return Err(());
    }

    let response: CloudflareResponse<CloudflareUserDetailsResponse> =
        response.json().await.map_err(|e| {
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

/// Command for getting all dns entries for a zone
///
/// # Errors
///
/// This will return an error if the token is invalid.
pub async fn get_zone_dns(
    token: &str,
    zone_id: String,
) -> Result<CloudflareResponse<Vec<DNSRecord>>, ()> {
    let authorizer = BearerAuthorizer {
        token: token.to_string(),
    };
    let client = reqwest::Client::new();

    let request_builder = client
        .get(format!(
            "{CLOUDFLARE_API_BASE}/zones/{zone_id}/dns_records?per_page=1000"
        ))
        .header("Content-Type", "application/json");

    let request_builder = authorizer.with_auth(request_builder);

    tracing::info!("get_zone_dns: Sending request to Cloudflare API for {zone_id}");

    let response: CloudflareResponse<Vec<DNSRecord>> = request_builder
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
