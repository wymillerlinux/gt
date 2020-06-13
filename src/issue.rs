use std::collections::HashMap;

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
        
    }
}
