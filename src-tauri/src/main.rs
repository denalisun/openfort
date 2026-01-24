// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::Path;

mod commands;
mod utils;
mod data;

use crate::commands::*;

fn main() {
    // Pre-launch setup
    let appdata_folder = Path::new(std::env::var("LOCALAPPDATA").unwrap().as_str()).join(".openfort");
    if !appdata_folder.is_dir() {
        // let _ = std::fs::create_dir(appdata_folder);
        match std::fs::create_dir(appdata_folder) {
            Ok(_) => {},
            Err(e) => {
                println!("Err creating appdata folder: {}", e.to_string());
            }
        }
    }

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