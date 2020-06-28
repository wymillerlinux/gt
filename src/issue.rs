use std::collections::HashMap;
use std::io;

use clap::ArgMatches;
use colored::*;
use reqwest::{StatusCode, Client};
use serde_derive::{Serialize, Deserialize};

use crate::config::Configuration;

pub struct Issue;

impl Issue {
    pub fn new() -> Issue {
        Issue {}
    }

    pub fn create_issue(&self, config: &Configuration, arg: &ArgMatches) {
        let client = Client::new();
    }

    pub fn get_input(&self) -> String {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }
}
