use std::collections::HashMap;

use clap::ArgMatches;
use reqwest::Client;

use crate::config::Configuration;

pub struct Request<'a> {
    pub client: Client,
    pub arg_value: Vec<&'a str>,
    pub map: HashMap<String, String>,
    pub url: Option<String>,
}

pub struct Authentication {
    pub basic: bool,
    pub api_token: bool,
}

impl Request<'static> {
    fn new() -> Request<'static> {
        Request {
            client: Client::new(),
            arg_value: Vec::new(),
            map: HashMap::new(),
            url: None
        }
    }
}

impl Authentication {
    pub fn new(config: &Configuration) -> Authentication {
        let basic_auth: String;
        let api_auth: String;

        // TODO: might be broken, haven't tested
        // this is horror code, I know it
        // match the damn thing
        // someone is going to take this and put it in r/badcode lol

        match config {
            _ => panic!()
        }
    }

    fn with_basic() -> Authentication {
        Authentication {
            basic: true,
            api_token: false
        }
    }

    fn with_api_token() -> Authentication {
        Authentication {
            basic: false,
            api_token: true
        }
    }

    // This method requires the instanciated config, an optional instanciated args, and instanciated auth
    pub fn request_chooser(&self, config: &Configuration, arg: Option<&ArgMatches>, auth: &Authentication, arg_string: &str) {

    }
}