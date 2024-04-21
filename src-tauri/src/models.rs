//! Data models for the application.

use std::{collections::HashMap, sync::Mutex};

use crate::cloudflare::{CloudflareListZonesResponse, DNSRecord};

/// Short details about the user, for display in the UI.
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
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

/// Managed cache of the queries that are supported
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ManagedCache {
    /// Cloudflare API token (bearer token)
    pub api_token: Mutex<String>,
    /// Zone IDs
    pub zones: Mutex<Vec<CloudflareListZonesResponse>>,
    /// Map of zone IDs to DNS records vectors
    pub zone_dns: Mutex<HashMap<String, Vec<DNSRecord>>>,
    /// User details (verify API key)
    pub user_details: Mutex<Option<CustomUserDetails>>,
}
