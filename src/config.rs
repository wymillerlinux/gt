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

        match env::consts::OS {
            // this case is currently untested
            "windows" => {
                location.push(String::from("config.json"));
                location.push(String::from("{:?}/AppData/gt/config.json"))
            },
            // this case is currently untested
            "macos" => {
                location.push(String::from("config.json"));
            },
            "linux" => {
                location.push(String::from("config.json"));
                location.push(String::from("/etc/gt/config.json"));
                location.push(format!("{:?}/.config/gt/config.json", home_dir_env));            
            },
            _ => {
                println!("Unsupported operating system! {:?} might cause some instabilities!", env::consts::OS);
                location.push(String::from("config.json"));
            }
        }

        for i in location {
            settings.merge(File::with_name(&i).required(false)).unwrap();
        }

        let config = settings
            .try_into::<Configuration>()
            .expect("Couldn't load config into gt!");

        config
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Self::new()
    }
}
