use std::collections::HashMap;

use clap::ArgMatches;
use colored::*;
use reqwest::{StatusCode, Client};
use serde_derive::{Serialize, Deserialize};

use crate::config::Configuration;
use crate::request::Request;

pub struct Repository;

#[derive(Debug, Serialize, Deserialize)]
pub struct MultipleRepositories {
    pub data: Vec<RepositoryResponse>,
    pub ok: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RepositoryResponse {
    pub allow_merge_commits: bool,
    pub allow_rebase: bool,
    pub allow_rebase_explicit: bool,
    pub allow_squash_merge: bool,
    pub archived: bool,
    pub avatar_url: String,
    pub clone_url: String,
    pub created_at: String,
    pub default_branch: String,
    pub description: String,
    pub empty: bool,
    pub external_tracker: Option<ExternalTracker>,
    pub external_wiki: Option<ExternalWiki>,
    pub fork: bool,
    pub forks_count: u32,
    pub full_name: String,
    pub has_issues: bool,
    pub has_pull_requests: bool,
    pub has_wiki: bool,
    pub html_url: String,
    pub id: u32,
    pub ignore_whitespace_conflicts: bool,
    pub internal_tracker: Option<InternalTracker>,
    pub mirror: bool,
    pub name: String,
    pub open_issues_count: u32,
    pub open_pr_counter: u32,
    pub original_url: String,
    pub owner: Option<Owner>,
    pub permissions: Option<Permissions>,
    pub private: bool,
    pub release_counter: u32,
    pub size: u32,
    pub ssh_url: String,
    pub stars_count: u32,
    pub template: bool,
    pub updated_at: String,
    pub watchers_count: u32,
    pub website: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalTracker {
    pub external_tracker_format: String,
    pub external_tracker_style: String,
    pub external_tracker_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalWiki {
    pub external_wiki_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InternalTracker {
    pub allow_only_contributors_to_track_time: bool,
    pub enable_issue_dependencies: bool,
    pub enable_time_tracker: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Owner {
    pub avatar_url: String,
    pub created: String,
    pub email: String,
    pub full_name: String,
    pub id: u32,
    pub is_admin: bool,
    pub language: String,
    pub last_login: String,
    pub login: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Permissions {
    pub admin: bool,
    pub pull: bool,
    pub push: bool,
}

impl Repository {
    pub fn new() -> Repository {
        Repository {}
    }

    pub fn create_repo(&self, request: &Request) {
        let client = &request.client;
        let arg_value = request.arg_value.subcommand().1.unwrap().value_of("create").unwrap();
        let mut map: HashMap<&str, &str> = HashMap::new();
        let url = format!("{request}/user/repos?token={api_token}",
            request = request.url.as_ref().unwrap(),
            api_token = request
                .authentication
                .credentials
                .1.as_ref()
                .unwrap());

        map.insert("name", arg_value);
        map.insert("readme", arg_value);
        map.insert("description", arg_value);

        let response = client.post(url.as_str())
            .json(&map)
            .send();

        match response {
            Ok(mut repo) => {
                match repo.status() {
                    StatusCode::CREATED => {
                            let deserialized: RepositoryResponse = repo.json().unwrap();
                            println!("{}", "Repository successfully created!".green());
                            println!("\tRepository name: {:0?}\n\tRepository owner: {:1?}\n\tRepository description: {:2?}", deserialized.name, deserialized.owner.unwrap().full_name, deserialized.description);
                        },
                    StatusCode::CONFLICT => println!("{}", "Repository already exists!".red()),
                    StatusCode::UNPROCESSABLE_ENTITY => println!("{}", "Repository input validation failed!".red()),
                    _ => println!("Repository creation failed! HTTP status code: {}", repo.status().as_str())
                }
            },
            Err(e) => panic!(e)
        }
    }

    pub fn delete_repo(&self, config: &Configuration, arg: &ArgMatches) {
        let client = Client::new();
        let arg_iter: Vec<&str> = arg.values_of("delete")
            .unwrap()
            .collect();

        let url = format!("{base_url}{base_api}/repos/{username}/{repo_name}?token={api_token}",
            base_url = config.base_url,
            base_api = config.base_api,
            username = arg_iter[0],
            repo_name = arg_iter[1],
            api_token = config.api_token.as_ref().unwrap());

        let response = client.delete(&url)
            .send();

        match response {
            Ok(repo) => {
                match repo.status() {
                    StatusCode::NO_CONTENT => println!("{}", "Repository successfully deleted!".green()),
                    StatusCode::FORBIDDEN => println!("{}", "Forbidden to delete this repository!".red()),
                    StatusCode::NOT_FOUND => println!("{}", "Repository doesn't exist!".red()),
                    _ => println!("Repository deletion failed! HTTP status code: {}", repo.status().to_string())
                }
            },
            Err(e) => panic!(e)
        }
    }

    pub fn fork_repo(&self, config: &Configuration, arg: &ArgMatches) {
        let client = Client::new();
        let arg_item: Vec<&str> = arg.values_of("fork")
            .unwrap()
            .collect();

        let mut map: HashMap<&str, &str> = HashMap::new();
        let user = config.username
            .clone()
            .unwrap();
        

        let url = format!("{base_url}{base_api}/repos/{owner}/{repo}/forks?token={api_token}",
            base_url = config.base_url,
            base_api = config.base_api,
            owner = arg_item[0],
            repo = arg_item[1],
            api_token = config.api_token.as_ref().unwrap());

        map.insert("name", user.as_str());

        let response = client.post(url.as_str())
            .json(&map)
            .send();

        match response {
            Ok(mut repo) => {
                match repo.status() {
                    StatusCode::ACCEPTED => {
                            let deserialized: RepositoryResponse = repo.json().unwrap();
                            println!("{}", "Repository forked successfully".green());
                            println!("\tOriginal repository name: {:0?}\n\tOriginal repository owner: {:1?}\n\tForked repository name: {:2?}\n\tForked repository owner: {:3?}", deserialized.name, arg_item[0], deserialized.name, deserialized.owner.unwrap().full_name);
                        },
                    StatusCode::INTERNAL_SERVER_ERROR => println!("{}", "Repository already forked!".red()),
                    StatusCode::FORBIDDEN => println!("{}", "Repository fork forbidden!".red()),
                    StatusCode::UNPROCESSABLE_ENTITY => println!("{}", "Repository fork input validation failed!".red()),
                    StatusCode::NOT_FOUND => println!("{}", "Repository not found!"),
                    _ => println!("Repository creation failed! HTTP status code: {}", repo.status().as_str())
                }
            },
            Err(e) => panic!(e)
        }
    }

    pub fn search_repo(&self, config: &Configuration, arg: &ArgMatches) {
        let client = Client::new();
        let arg_value = arg.value_of("search").unwrap();
        let url = format!("{base_url}{base_api}/repos/search?q={query}&token={api_token}",
            base_url = config.base_url,
            base_api = config.base_api,
            query = arg_value,
            api_token = config.api_token.as_ref().unwrap());

        let response = client.get(url.as_str())
            .send();

        match response {
            Ok(mut repo) => {
                match repo.status() {
                    StatusCode::OK => {
                            let deserialized: MultipleRepositories = repo.json()
                                .unwrap();

                            match deserialized.data.len() != 0 {
                                true => {
                                    println!("{}", "List of repositories found:".green());

                                    for (i, data) in deserialized.data.iter().enumerate() {
                                        println!("{}.\tRepository name: {:1?}\n\tRepository owner: {:2?}\n\tRepository description: {:3?}\n", i + 1, data.name, data.owner.as_ref().unwrap().full_name, data.description)
                                    }

                                    println!("Total number of repositories indexed: {}", deserialized.data.iter().count())
                                },
                                false => println!("{}", "Repository searched doesn't exist!".red())
                            }
                        },
                    StatusCode::NOT_FOUND => println!("{}", "Repository searched doesn't exist!".red()),
                    StatusCode::UNPROCESSABLE_ENTITY => println!("{}", "Repository input validation failed!".red()),
                    _ => println!("Repository search failed! HTTP status code: {}", repo.status().as_str())
                }
            },
            Err(e) => panic!(e)
        }
    }

    pub fn list_repo(&self, config: &Configuration) {
        let client = Client::new();
        let url = format!("{base_url}{base_api}/repos/search?token={api_token}",
            base_url = config.base_url,
            base_api = config.base_api,
            api_token = config.api_token.as_ref().unwrap());

        let response = client.get(url.as_str())
            .send();

        match response {
            Ok(mut repo) => {
                match repo.status() {
                    StatusCode::OK => {
                            let deserialized: MultipleRepositories = repo.json()
                                .unwrap();

                            match deserialized.data.len() != 0 {
                                true => {
                                    println!("{}", "List of repositories found:".green());

                                    for (i, data) in deserialized.data.iter().enumerate() {
                                        println!("{}.\tRepository name: {:1?}\n\tRepository owner: {:2?}\n\tRepository description: {:3?}\n", i + 1, data.name, data.owner.as_ref().unwrap().full_name, data.description)
                                    }

                                    println!("Total number of repositories indexed: {}", deserialized.data.iter().count())
                                },
                                false => println!("{}", "The authenticated user doesn't have any repositories. Why not create one?".yellow())
                            }
                        },
                    StatusCode::UNPROCESSABLE_ENTITY => println!("{}", "Repository input validation failed!".red()),
                    _ => println!("Repository search failed! HTTP status code: {}", repo.status().as_str())
                }
            },
            Err(e) => panic!(e)
        }
    }    
}
