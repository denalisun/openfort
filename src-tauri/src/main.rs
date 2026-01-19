// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod utils;
mod data;

use windows::Win32::System::Threading::{CREATE_NEW_PROCESS_GROUP, DETACHED_PROCESS};
use crate::utils::*;
use crate::data::*;

use std::{io::{BufRead, BufReader}, os::windows::process::CommandExt, path::Path, process::{Child, Command, Stdio}};

#[tauri::command]
fn validate_install(path: String) -> bool {
    let actual_path: &Path = Path::new(&path);
    if actual_path.join("FortniteGame").is_dir() && actual_path.join("Engine").is_dir() {
        return true;
    }
    false
}

#[tauri::command]
fn launch_editor(path: String) {
    std::thread::spawn(move || {
        let args = format!("-epicapp=Fortnite -epicenv=Prod -epiclocale=en-us -epicportal -skippatchcheck -NOSSLPINNING -nobe -fromfl=eac -fltoken=3db3ba5dcbd2e16703f3978d -caldera=eyJhbGciOiJFUzI1NiIsInR5cCI6IkpXVCJ9.eyJhY2NvdW50X2lkIjoiYmU5ZGE1YzJmYmVhNDQwN2IyZjQwZWJhYWQ4NTlhZDQiLCJnZW5lcmF0ZWQiOjE2Mzg3MTcyNzgsImNhbGRlcmFHdWlkIjoiMzgxMGI4NjMtMmE2NS00NDU3LTliNTgtNGRhYjNiNDgyYTg2IiwiYWNQcm92aWRlciI6IkVhc3lBbnRpQ2hlYXQiLCJub3RlcyI6IiIsImZhbGxiYWNrIjpmYWxzZX0.VAWQB67RTxhiWOxx7DBjnzDnXyyEnX7OljJm-j2d88G_WgwQ9wrE6lwMEHZHjBd1ISJdUO1UVUqkfLdU5nofBQ -AUTH_LOGIN={}@. -AUTH_PASSWORD=ilovejosephstalin -AUTH_TYPE=epic -disableplugins=\"AtomVK,ValkyrieFortnite\"", "OPENFORT-EDITOR");
        let launch_args: Vec<&str> = args.split_whitespace().collect();

        let fortnite_binaries = Path::new(path.as_str()).join("FortniteGame\\Binaries\\Win64");
        let uefn_path = fortnite_binaries.clone().join("UnrealEditorFortnite-Win64-Shipping.exe");
        println!("uefn_path: {}", uefn_path.to_str().unwrap());

        let uefn_patched_path = fortnite_binaries.clone().join("OpenFort-UEFN.exe");
        if !uefn_patched_path.is_file() {
            let _ = std::fs::File::create(&uefn_patched_path);
            match std::fs::copy(uefn_path, &uefn_patched_path) {
                Ok(_) => {
                    match patch_uefn(uefn_patched_path.clone()) {
                        Ok(_) => {
                            println!("Successfully patched UEFN!");
                        },
                        Err(e) => {
                            println!("Err: {}", e);
                            return;
                        }
                    };
                },
                Err(e) => {
                    println!("Err: {}", e);
                    return;
                },
            }
        }

        let appdata_folder = Path::new(std::env::var("LOCALAPPDATA").unwrap().as_str()).join(".openfort");
        let redirector_dll = appdata_folder.join("ssl.dll");
        
        let engine_binaries = Path::new(path.as_str()).join("Engine\\Binaries\\Win64");
        let uba_dir = engine_binaries.join("UnrealBuildAccelerator");
        if !uba_dir.is_dir() {
            match std::fs::create_dir(&uba_dir) {
                Ok(_) => {},
                Err(e) => {
                    println!("Err while creating SSL bypass injection point: {}", e.to_string());
                    return;
                }
            }
        }

        let uba_x64_dir = uba_dir.join("x64");
        if !uba_x64_dir.is_dir() {
            match std::fs::create_dir(&uba_x64_dir) {
                Ok(_) => {},
                Err(e) => {
                    println!("Err while creating SSL bypass injection point: {}", e.to_string());
                    return;
                }
            }
        }

        let ubahost_dll_path = Path::new(path.as_str()).join("Engine\\Binaries\\Win64\\UnrealBuildAccelerator\\x64");
        if !ubahost_dll_path.clone().is_dir() {
            match std::fs::create_dir(&ubahost_dll_path) {
                Ok(_) => {},
                Err(e) => {
                    println!("Err while creating SSL bypass injection point: {}", e.to_string());
                    return;
                }
            }
        }

        let ubahost_dll = ubahost_dll_path.join("UbaHost.dll");
        if !ubahost_dll.is_file() {
            match std::fs::copy(redirector_dll, ubahost_dll) {
                Ok(_) => {},
                Err(e) => {
                    println!("Err while copying SSL bypass to injection point: {}", e.to_string());
                    return;
                }
            };
        }

        let creation_flags = (DETACHED_PROCESS | CREATE_NEW_PROCESS_GROUP).0;

        Command::new(uefn_patched_path)
            .current_dir(&fortnite_binaries)
            .args(&launch_args)
            .creation_flags(creation_flags)
            .spawn()
            .unwrap();
    });
}

#[tauri::command]
fn launch_install(path: String, username: String, is_server: bool) {
    std::thread::spawn(move || {
        let args = format!("-epicapp=Fortnite -epicenv=Prod -epiclocale=en-us -epicportal -skippatchcheck -NOSSLPINNING -nobe -fromfl=eac -fltoken=3db3ba5dcbd2e16703f3978d -caldera=eyJhbGciOiJFUzI1NiIsInR5cCI6IkpXVCJ9.eyJhY2NvdW50X2lkIjoiYmU5ZGE1YzJmYmVhNDQwN2IyZjQwZWJhYWQ4NTlhZDQiLCJnZW5lcmF0ZWQiOjE2Mzg3MTcyNzgsImNhbGRlcmFHdWlkIjoiMzgxMGI4NjMtMmE2NS00NDU3LTliNTgtNGRhYjNiNDgyYTg2IiwiYWNQcm92aWRlciI6IkVhc3lBbnRpQ2hlYXQiLCJub3RlcyI6IiIsImZhbGxiYWNrIjpmYWxzZX0.VAWQB67RTxhiWOxx7DBjnzDnXyyEnX7OljJm-j2d88G_WgwQ9wrE6lwMEHZHjBd1ISJdUO1UVUqkfLdU5nofBQ -AUTH_LOGIN={}@. -AUTH_PASSWORD=somethingmoreappropriate -AUTH_TYPE=epic", if username != "" { username } else { "UnknownLooper".to_string() });
        let mut launch_args: Vec<&str> = args.split_whitespace().collect();

        let fortnite_binaries = Path::new(path.as_str()).join("FortniteGame\\Binaries\\Win64");
        let fortnite_launcher_path = fortnite_binaries.clone().as_path().join("FortniteLauncher.exe");
        let fortnite_eac_path = fortnite_binaries.clone().as_path().join("FortniteClient-Win64-Shipping_EAC.exe");
        let fortnite_client_path = fortnite_binaries.clone().as_path().join("FortniteClient-Win64-Shipping.exe");
        let fortnite_server_path = fortnite_binaries.clone().as_path().join("OpenFort-Server.exe");

        if is_server {
            // Im just gonna assume that you wanna run headless when running a server lowk
            // Ill add a toggle for it later but im not Auties00 so maybe not
            if !fortnite_server_path.is_file() {
                match std::fs::copy(&fortnite_client_path, &fortnite_server_path) {
                    Ok(_) => {
                        match patch_for_server(&fortnite_server_path) {
                            Ok(_) => {
                                println!("Successfully patched server!");
                            },
                            Err(e) => {
                                println!("Err while patching Fortnite client: {}", e);
                                return;
                            }
                        }
                    },
                    Err(e) => {
                        println!("Err while patching Fortnite client: {}", e);
                        return;
                    }
                }
            }

            launch_args.push("-nullrhi");
        }

        let creation_flags = (DETACHED_PROCESS | CREATE_NEW_PROCESS_GROUP).0;

        let mut launcher_process: Child = Command::new(fortnite_launcher_path)
            .current_dir(&fortnite_binaries)
            .args(&launch_args)
            .creation_flags(creation_flags)
            .spawn()
            .unwrap();
        nt_suspend_process(launcher_process.id());

        let mut eac_process: Child = Command::new(fortnite_eac_path)
            .current_dir(&fortnite_binaries)
            .args(&launch_args)
            .creation_flags(creation_flags)
            .spawn()
            .unwrap();
        nt_suspend_process(eac_process.id());

        let mut fortnite_process: Child = Command::new(if is_server { fortnite_server_path } else { fortnite_client_path })
            .current_dir(&fortnite_binaries)
            .args(&launch_args)
            .creation_flags(creation_flags)
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let appdata_folder = Path::new(std::env::var("LOCALAPPDATA").unwrap().as_str()).join(".openfort");
        let redirector_dll = appdata_folder.join("ssl.dll");
        println!("Attempting to inject SSL bypass...");
        match inject_dll(fortnite_process.id(), redirector_dll.to_str().unwrap()) {
            Ok(_) => {
                println!("Successfully injected SSL bypass!");
            },
            Err(e) => {
                let _ = fortnite_process.kill();
                let _ = eac_process.kill();
                let _ = launcher_process.kill();

                println!("Err while injecting SSL bypass: {}", e.to_string());
                return;
            }
        };

        // Injecting dlls at login point
        let fortnite_stdout = fortnite_process.stdout.take().expect("stdout missing");
        let fortnite_process_id = fortnite_process.id();
        std::thread::spawn(move || {
            let reader = BufReader::new(fortnite_stdout);
            for line in reader.lines() {
                let unwrapped_line = line.unwrap();
                if unwrapped_line.contains("[UOnlineAccountCommon::ContinueLoggingIn]") && unwrapped_line.contains("(Completed)") {
                    if is_server {
                        let server_dll = appdata_folder.join("server.dll");
                        match inject_dll(fortnite_process_id, server_dll.to_str().unwrap()) {
                            Ok(_) => {
                                println!("Successfully injected server DLL!");
                            },
                            Err(e) => {
                                println!("Err while injecting server: {}", e.to_string());
                                return;
                            }
                        }
                    } else {
                        let client_dll = appdata_folder.join("client.dll");
                        match inject_dll(fortnite_process_id, client_dll.to_str().unwrap()) {
                            Ok(_) => {
                                println!("Successfully injected client DLL!");
                            },
                            Err(e) => {
                                println!("Err while injecting client: {}", e.to_string());
                                return;
                            }
                        }
                    }
                }
            }
        });

        let _ = fortnite_process.wait();

        let _ = eac_process.kill();
        let _ = launcher_process.kill();
    });
}

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

    let settings: AppSettings = AppSettings::new("", "", "");

    // TODO: Check for args
    // example: --username=EFortRarity --path="C:\\OpenFortBuild

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            validate_install,
            launch_editor,
            launch_install
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}