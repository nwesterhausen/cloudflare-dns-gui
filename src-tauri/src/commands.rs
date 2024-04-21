//! This module contains the Tauri commands that are exposed to the JavaScript side of the application.

use std::collections::HashMap;

use tauri::State;

use crate::{
    api,
    cloudflare::{CloudflareListZonesResponse, DNSRecord},
    models::{CustomUserDetails, ManagedCache},
};

/// Set the api_token
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub async fn set_api_token(
    token: String,
    managed_cache: State<'_, ManagedCache>,
) -> Result<(), ()> {
    // Clear the cache
    {
        managed_cache.zones.lock().unwrap().clear();
        managed_cache.zone_dns.lock().unwrap().clear();
        *managed_cache.user_details.lock().unwrap() = None;
        *managed_cache.api_token.lock().unwrap() = String::default();
    }
    *managed_cache.api_token.lock().unwrap() = token.clone();

    // Check the token is valid
    let user_details = api::check_api_key(&token).await;
    if let Ok(user_details) = user_details {
        // Update the user details and re-lock the cache
        {
            *managed_cache.user_details.lock().unwrap() = Some(user_details);
        }
    }
    Ok(())
}

/// Initialize the Cloudflare API with the provided token.
/// This will clear the cache and set the token, for use in subsequent requests.
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub async fn initialize_cf(managed_cache: State<'_, ManagedCache>) -> Result<bool, ()> {
    // Clear the cache
    {
        managed_cache.zones.lock().unwrap().clear();
        managed_cache.zone_dns.lock().unwrap().clear();
        *managed_cache.user_details.lock().unwrap() = None;
    }
    // Attempt to set the token
    let new_token = managed_cache.api_token.lock().unwrap().clone();
    // Check the token is valid
    let user_details = api::check_api_key(&new_token).await;
    if let Ok(user_details) = user_details {
        // Update the user details and re-lock the cache
        {
            *managed_cache.user_details.lock().unwrap() = Some(user_details);
        }
        // Get the zones and DNS records
        let zones = match api::get_zones(&new_token).await {
            Ok(zones) => {
                if zones.success {
                    zones.result
                } else {
                    return Err(());
                }
            }
            Err(_) => return Err(()),
        };
        // Update the cache with the zone details
        {
            *managed_cache.zones.lock().unwrap() = zones.clone();
        }
        let zone_ids: Vec<String> = zones.iter().map(|z| z.id.clone()).collect();

        {
            for zone_id in &zone_ids {
                let dns_records = api::get_zone_dns(&new_token, zone_id.clone())
                    .await
                    .unwrap();
                if !dns_records.success {
                    return Err(());
                }
                let dns_records = dns_records.result;
                let mut zone_dns = managed_cache.zone_dns.lock().unwrap();

                zone_dns.insert(zone_id.clone(), dns_records);
            }
        }
    }
    Ok(true)
}

/// Check if the API key was valid.
/// This will return the user details if the key is valid, or an error if it is not.
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub async fn get_user_details(
    managed_cache: State<'_, ManagedCache>,
) -> Result<CustomUserDetails, ()> {
    if let Some(user_details) = &*managed_cache.user_details.lock().unwrap() {
        Ok(user_details.clone())
    } else {
        Err(())
    }
}

/// Get the zones for the current user. This is pulled from the cache.
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub async fn get_zones(
    managed_cache: State<'_, ManagedCache>,
) -> Result<Vec<CloudflareListZonesResponse>, ()> {
    if let Ok(zones) = managed_cache.zones.lock() {
        return Ok(zones.clone());
    }
    Err(())
}

/// Get the DNS records for a zone. This is pulled from the cache.
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
