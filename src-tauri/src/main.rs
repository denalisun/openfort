// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::Path;

mod commands;
mod utils;
mod data;

use crate::{commands::*, data::AppSettings};

fn main() {
    // Pre-launch setup
    let appdata_folder = Path::new(std::env::var("LOCALAPPDATA").unwrap().as_str()).join(".openfort");
    if !appdata_folder.is_dir() {
        // let _ = std::fs::create_dir(appdata_folder);
        match std::fs::create_dir(appdata_folder.clone()) {
            Ok(_) => {},
            Err(e) => {
                panic!("Err creating appdata folder: {}", e.to_string());
            }
        }
    }

    match std::fs::exists(appdata_folder.join("settings.json")) {
        Ok(b) => {
            if !b {
                let base_settings: AppSettings = AppSettings::new("", "", "");
                let settings_json = serde_json::to_string(&base_settings).unwrap();
                match std::fs::write(appdata_folder.join("settings.json"), settings_json) {
                    Ok(_) => {},
                    Err(e) => {
                        panic!("Err creating settings JSON: {}", e.to_string());
                    }
                };
            }
        },
        Err(e) => {
            panic!("Err finding settings JSON: {}", e.to_string());
        }
    };

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            validate_install,
            launch_editor,
            launch_install,
            change_settings,
            read_settings,
            does_build_have_uefn
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}