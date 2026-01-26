use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AppSettings {
    pub fortnite_path: String,
    pub username: String,
    pub extra_launch_args: String,
}

impl AppSettings {
    pub fn new(fortnite_path: &str, username: &str, extra_launch_args: &str) -> Self {
        Self {
            fortnite_path: fortnite_path.to_string(),
            username: username.to_string(),
            extra_launch_args: extra_launch_args.to_string(),
        }
    }
}

#[allow(dead_code)]
pub struct FortniteInstall {
    pub has_uefn: bool,
    pub has_battleye: bool,
    pub has_easyanticheat: bool,
    pub has_launcher: bool,
}

#[allow(dead_code)]
impl FortniteInstall {
    pub fn default() -> Self {
        Self {
            has_uefn: false,
            has_battleye: false,
            has_easyanticheat: false,
            has_launcher: false,
        }
    }

    pub fn from_path(path: &Path) -> Self {
        let mut install: FortniteInstall = FortniteInstall::default();

        let fortnite_binaries = path.join("FortniteGame\\Binaries\\Win64");
        let fortnite_launcher_path = fortnite_binaries
            .clone()
            .as_path()
            .join("FortniteLauncher.exe");
        let fortnite_be_path = fortnite_binaries
            .clone()
            .as_path()
            .join("FortniteClient-Win64-Shipping_BE.exe");
        let fortnite_eac_path = fortnite_binaries
            .clone()
            .as_path()
            .join("FortniteClient-Win64-Shipping_EAC.exe");
        let uefn_path = fortnite_binaries
            .clone()
            .as_path()
            .join("UnrealEditorFortnite-Win64-Shipping.exe");

        if fortnite_launcher_path.is_file() {
            install.has_launcher = true;
        }

        if fortnite_be_path.is_file() {
            install.has_battleye = true;
        }

        if fortnite_eac_path.is_file() {
            install.has_easyanticheat = true;
        }

        if uefn_path.is_file() {
            install.has_uefn = true;
        }

        install
    }
}
