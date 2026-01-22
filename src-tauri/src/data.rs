use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AppSettings {
    pub fortnite_path: String,
    pub username: String,
    pub extra_launch_args: String,
}

impl AppSettings {
    pub fn new(fortnite_path: &str, username: &str, extra_launch_args: &str) -> Self {
        Self { fortnite_path: fortnite_path.to_string(), username: username.to_string(), extra_launch_args: extra_launch_args.to_string() }
    }
}