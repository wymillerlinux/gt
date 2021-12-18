use std::collections::HashMap;

use clap::ArgMatches;
use reqwest::blocking::Client;

use crate::config::Configuration;

pub struct Request<'a> {
    pub client: Client,
    pub arg_value: ArgMatches<'a>,
    pub map: HashMap<String, String>,
    pub url: Option<String>,
    pub authentication: Authentication,
}

pub struct Authentication {
    pub basic: bool,
    pub api_token: bool,
    pub credentials: (Option<String>, Option<String>),
}

impl<'a> Request<'a> {
    /// Public constructor for a request with a simple username and password
    pub fn with_basic_request(
        config: Configuration,
        arg: ArgMatches,
        auth: Authentication,
    ) -> Request {
        Request {
            client: Client::new(),
            arg_value: arg,
            map: HashMap::new(),
            url: Some(format!("{}{}", config.base_url, config.base_api)),
            authentication: auth,
        }
    }

    /// Public constructor for a request with an API token
    pub fn with_api_request(
        config: Configuration,
        arg: ArgMatches,
        auth: Authentication,
    ) -> Request {
        Request {
            client: Client::new(),
            arg_value: arg,
            map: HashMap::new(),
            url: Some(format!("{}{}", config.base_url, config.base_api)),
            authentication: auth,
        }
    }
}

impl Authentication {
    /// Public constructor for getting authentication, provided by the configuration
    /// file. The most secure methods are checked first, filtering down to the least
    /// secure methods. Currently, only two methods are supported: API token, and
    /// username/password combo.
    pub fn new(config: &Configuration) -> Authentication {
        // TODO: might be broken, haven't tested
        // this is horror code, I know it
        // match the damn thing
        // someone is going to take this and put it in r/badcode lol
        let basic_auth = config.password.as_ref().unwrap().to_string();

        let api_auth = config.api_token.as_ref().unwrap().to_string();

        if api_auth.is_empty() {
            if !(basic_auth.is_empty()) {
                Authentication::with_basic(
                    config.username.as_ref().unwrap().to_string(),
                    basic_auth,
                )
            } else {
                panic!("Must have some form of authentication! Exiting...");
            }
        } else {
            Authentication::with_api_token(config.username.as_ref().unwrap().to_string(), api_auth)
        }
    }

    /// Private constructor once the public constructor figures out what kind of authentication
    /// is being used. This constructor uses the username/password combo, a less secure of
    /// authenticating that the API token.
    fn with_basic(user: String, pass: String) -> Authentication {
        Authentication {
            basic: true,
            api_token: false,
            credentials: (Some(user), Some(pass)),
        }
    }

    /// Private constructor once the public constructor figures out what kind of authentication
    /// is being used. This constructor uses the API token, a more secure way of authenticating
    /// instead of using the basic username and password.
    fn with_api_token(user: String, api_token: String) -> Authentication {
        Authentication {
            basic: false,
            api_token: true,
            credentials: (Some(user), Some(api_token)),
        }
    }

    /// Public method that based on the what kind of authentication is being used, it can
    /// determine what kind of requesting method is going to be used. See the Request
    /// structure for more details.
    pub fn request_chooser(self, config: Configuration, arg: ArgMatches<'static>) -> Request {
        if let true = self.api_token {
            Request::with_api_request(config, arg.to_owned(), self)
        } else {
            match self.basic {
                true => Request::with_basic_request(config, arg.to_owned(), self),
                false => panic!(),
            }
        }
    }
}
