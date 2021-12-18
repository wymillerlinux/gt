use std::env;

use config::File;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Configuration {
    pub api_token: Option<String>,
    pub base_api: String,
    pub base_url: String,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl Configuration {
    pub fn new() -> Configuration {
        let home_dir_env = env::var("HOME").unwrap();
        let mut settings = config::Config::default();
        let mut location: Vec<String> = Vec::new();

        // TODO: add condition for target os
        location.push("config.json".to_string());
        location.push("/etc/gt/config.json".to_string());
        location.push(format!("{}/.config/gt/config.json", home_dir_env));

        for i in location {
            settings.merge(File::with_name(&i).required(false)).unwrap();
        }

        let config = settings
            .try_into::<Configuration>()
            .expect("Couldn't load config into gt!");

        config
    }
}

#[cfg(target_os = "linux")]
fn set_location_linux(location: &mut Vec<String>, home: String) -> Vec<String> {
    location.push("config.json".to_string());
    location.push("/etc/gt/config.json".to_string());
    location.push(format!("{}/.config/gt/config.json", home));

    location.to_vec()
}

#[cfg(target_os = "macos")]
fn set_location_macos(location: &mut Vec<String>, home: String) -> Vec<String> {
    location.push("config.json".to_string());

    location.to_vec()
}

#[cfg(target_os = "windows")]
fn set_location_windows(location: &mut Vec<String>, home: String) -> Vec<String> {
    location.push("config.json".to_string());

    location.to_vec()
}