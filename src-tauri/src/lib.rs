//! A Tauri app to provide a fast and responsive UI for changing cloudflare DNS.

use std::sync::Mutex;

use models::ManagedCache;

#[allow(clippy::used_underscore_binding)]
pub mod api;
pub mod cloudflare;
pub mod commands;
pub mod models;

/// Entry point for the Tauri application.
///
/// # Panics
///
/// This will panic if the app fails to run
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::list_zones,
            commands::get_user_details,
            commands::get_zone_dns,
            commands::initialize_cf,
            commands::set_api_token,
        ])
        .manage(ManagedCache {
            zones: Mutex::default(),
            zone_dns: Mutex::default(),
            api_token: Mutex::default(),
            user_details: Mutex::default(),
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
