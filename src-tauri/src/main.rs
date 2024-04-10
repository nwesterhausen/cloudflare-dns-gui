//! A Tauri app to provide a fast and responsive UI for changing cloudflare DNS.
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod cloudflare;
#[allow(clippy::used_underscore_binding)]
pub mod commands;
pub mod models;

fn main() {
    tracing_subscriber::fmt::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::get_zones,
            commands::check_api_key
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
