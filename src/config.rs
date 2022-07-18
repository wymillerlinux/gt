use std::env;
use std::ops::Deref;

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
                location.push(format!("{:?}/AppData/Roaming/gt/config.json", home_dir_env))
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
    
    pub fn load_envs(&mut self) {
        // get environment variables
        let username_env = env::var("GT_USERNAME").unwrap_or_else(|_| "".to_string());
        let password_env = env::var("GT_PASSWORD").unwrap_or_else(|_| "".to_string());
        let api_token_env = env::var("GT_API_TOKEN").unwrap_or_else(|_| "".to_string());

        // get struct fields
        let mut user_config = self.username.as_ref().unwrap();
        let mut password_config = self.password.as_ref().unwrap();
        let mut api_token_config = self.api_token.as_ref().unwrap();

        // check and see if the env vars are empty
        // if they are not, put the env vars in place of the config property
        if username_env != "".to_string() {
            self.username = Some(username_env);
        } else {
            println!("cannot find username env var");
        }
 
        if password_env != "".to_string() {
            password_config = &password_env.deref().to_string();
        } else {
            println!("cannot find password env var");
        }

        if api_token_env != "".to_string() {
            api_token_config = &api_token_env.deref().to_string();
        } else {
            println!("cannot find api token env var");
        }

        println!("{:?}", &self);

    }
}

impl Default for Configuration {
    fn default() -> Self {
        Self::new()
    }
}
