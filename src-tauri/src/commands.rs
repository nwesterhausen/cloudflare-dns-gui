//! This module contains the Tauri commands that are exposed to the JavaScript side of the application.

use std::collections::HashMap;

use tauri::State;

use crate::{
    api,
    cloudflare::{CloudflareListZonesResponse, DNSRecord},
    models::{CustomUserDetails, ManagedCache},
};

/// Set the api_token
///
/// # Errors
///
/// This will return an error if the token is invalid.
///
/// It will also error if there's an issue accessing the cache.
///
/// # Panics
///
/// This will panic if the cache is poisoned.
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub async fn set_api_token(
    token: String,
    managed_cache: State<'_, ManagedCache>,
) -> Result<(), ()> {
    // Clear the cache
    #[allow(clippy::unwrap_used)]
    {
        managed_cache.zones.lock().unwrap().clear();
        managed_cache.zone_dns.lock().unwrap().clear();
        *managed_cache.user_details.lock().unwrap() = None;
        *managed_cache.api_token.lock().unwrap() = String::default();
    }
    // Set the token
    #[allow(clippy::unwrap_used)]
    {
        *managed_cache.api_token.lock().unwrap() = token.clone();
    }

    Ok(())
}

/// Initialize the Cloudflare API with the provided token.
/// This will clear the cache and set the token, for use in subsequent requests.
///
/// # Errors
///
/// This will return an error if the token is invalid.
///
/// It will also error if there's an issue accessing the cache.
///
/// # Panics
///
/// This will panic if the cache is poisoned.
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub async fn initialize_cf(managed_cache: State<'_, ManagedCache>) -> Result<bool, ()> {
    // Clear the cache
    #[allow(clippy::unwrap_used)]
    {
        managed_cache.zones.lock().unwrap().clear();
        managed_cache.zone_dns.lock().unwrap().clear();
        *managed_cache.user_details.lock().unwrap() = None;
    }
    // Attempt to set the token
    #[allow(clippy::unwrap_used)]
    let new_token = managed_cache.api_token.lock().unwrap().clone();
    // Check the token is valid
    let user_details = api::check_api_key(&new_token).await;
    if let Ok(user_details) = user_details {
        // Update the user details and re-lock the cache
        #[allow(clippy::unwrap_used)]
        {
            *managed_cache.user_details.lock().unwrap() = Some(user_details);
        }
        // Get the zones and DNS records
        let zones = match api::get_zones(&new_token).await {
            Ok(zones) => {
                if zones.success {
                    zones.result
                } else {
                    return Ok(false);
                }
            }
            Err(()) => return Ok(false),
        };
        // Update the cache with the zone details
        #[allow(clippy::unwrap_used)]
        {
            *managed_cache.zones.lock().unwrap() = zones.clone();
        }
        let zone_ids: Vec<String> = zones.iter().map(|z| z.id.clone()).collect();

        {
            for zone_id in &zone_ids {
                let Ok(dns_records) = api::get_zone_dns(&new_token, zone_id.clone()).await else {
                    return Ok(false);
                };
                if !dns_records.success {
                    return Ok(false);
                }
                let dns_records = dns_records.result;
                #[allow(clippy::unwrap_used)]
                let mut zone_dns = managed_cache.zone_dns.lock().unwrap();

                zone_dns.insert(zone_id.clone(), dns_records);
            }
        }
        return Ok(true);
    }
    Ok(false)
}

/// Check if the API key was valid.
/// This will return the user details if the key is valid, or an error if it is not.
///
/// # Errors
///
/// This will return an error if the token is invalid.
///
/// It will also error if there's an issue accessing the cache.
///
/// # Panics
///
/// This will panic if the cache is poisoned.
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub async fn get_user_details(
    managed_cache: State<'_, ManagedCache>,
) -> Result<CustomUserDetails, ()> {
    #[allow(clippy::unwrap_used)]
    (*managed_cache.user_details.lock().unwrap())
        .as_ref()
        .map_or(Err(()), |user_details| Ok(user_details.clone()))
}

/// Get the zones for the current user. This is pulled from the cache.
///
/// # Errors
///
/// This will return an error if the token is invalid.
///
/// It will also error if there's an issue accessing the cache.
///
/// # Panics
///
/// This will panic if the cache is poisoned.
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub async fn list_zones(
    managed_cache: State<'_, ManagedCache>,
) -> Result<Vec<CloudflareListZonesResponse>, ()> {
    if let Ok(zones) = managed_cache.zones.lock() {
        return Ok(zones.clone());
    }
    Err(())
}

/// Get the DNS records for a zone. This is pulled from the cache.
///
/// # Errors
///
/// This will return an error if the token is invalid.
///
/// It will also error if there's an issue accessing the cache.
///
/// # Panics
///
/// This will panic if the cache is poisoned.
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub async fn get_zone_dns(
    managed_cache: State<'_, ManagedCache>,
) -> Result<HashMap<String, Vec<DNSRecord>>, ()> {
    if let Ok(zone_dns) = managed_cache.zone_dns.lock() {
        return Ok(zone_dns.clone());
    }
    Err(())
}
