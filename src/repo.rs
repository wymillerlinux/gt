use std::collections::HashMap;

use colored::*;
use git2::Repository as Repo;
use reqwest::StatusCode;
use serde_derive::{Deserialize, Serialize};

use crate::{request::Request, config::Configuration};

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
        let arg_value = request
            .arg_value
            .subcommand()
            .1
            .unwrap()
            .value_of("create")
            .unwrap();
        let mut map: HashMap<&str, &str> = HashMap::new();
        let url = format!(
            "{request}/user/repos?token={api_token}",
            request = request.url.as_ref().unwrap(),
            api_token = request.authentication.credentials.1.as_ref().unwrap()
        );

        map.insert("name", arg_value);
        map.insert("readme", arg_value);
        map.insert("description", arg_value);

        let response = client.post(url.as_str()).json(&map).send();

        match response {
            Ok(repo) => match repo.status() {
                StatusCode::CREATED => {
                    let deserialized: RepositoryResponse = repo.json().unwrap();
                    println!("{}", "Repository successfully created!".green());
                    println!("\tRepository name: {:0}\n\tRepository owner: {:1}\n\tRepository description: {:2}", deserialized.name, deserialized.owner.unwrap().full_name, deserialized.description);
                }
                StatusCode::CONFLICT => println!("{}", "Repository already exists!".red()),
                StatusCode::UNPROCESSABLE_ENTITY => {
                    println!("{}", "Repository input validation failed!".red())
                }
                _ => println!(
                    "Repository creation failed! HTTP status code: {}",
                    repo.status().as_str()
                ),
            },
            Err(e) => panic!("{}", e),
        }
    }

    pub fn delete_repo(&self, request: &Request) {
        let client = &request.client;
        let arg_value: Vec<&str> = request
            .arg_value
            .subcommand()
            .1
            .unwrap()
            .values_of("delete")
            .unwrap()
            .collect();
        let url = format!(
            "{request}/repos/{owner}/{repo}?token={api_token}",
            request = request.url.as_ref().unwrap(),
            owner = request.authentication.credentials.0.as_ref().unwrap(),
            repo = arg_value[1],
            api_token = request.authentication.credentials.1.as_ref().unwrap(),
        );

        let response = client.delete(&url).send();

        match response {
            Ok(repo) => match repo.status() {
                StatusCode::NO_CONTENT => println!("{}", "Respository successfully deleted!".green()),
                StatusCode::FORBIDDEN => println!("{}", "Repository deletion forbidden!".red()),
                _ => println!(
                    "Repository deletion failed! Does the repository exist? HTTP status code: {}",
                    repo.status().as_str()
                ),
            },
            Err(e) => panic!("{}", e),
        }
    }

    pub fn fork_repo(&self, request: &Request) {
        let client = &request.client;
        let arg_item: Vec<&str> = request
            .arg_value
            .subcommand()
            .1
            .unwrap()
            .values_of("fork")
            .unwrap()
            .collect();

        let mut map: HashMap<&str, &str> = HashMap::new();
        let user = request.authentication.credentials.0.as_ref().unwrap();

        let url = format!(
            "{request}/repos/{owner}/{repo}/forks?token={api_token}",
            request = request.url.as_ref().unwrap(),
            owner = arg_item[0],
            repo = arg_item[1],
            api_token = request.authentication.credentials.1.as_ref().unwrap(),
        );

        map.insert("name", user.as_str());

        let response = client.post(url.as_str()).json(&map).send();

        match response {
            Ok(repo) => match repo.status() {
                StatusCode::ACCEPTED => {
                    let deserialized: RepositoryResponse = repo.json().unwrap();
                    println!("{}", "Repository forked successfully".green());
                    println!("\tOriginal repository name: {:0?}\n\tOriginal repository owner: {:1?}\n\tForked repository name: {:2?}\n\tForked repository owner: {:3?}", deserialized.name, arg_item[0], deserialized.name, deserialized.owner.unwrap().full_name);
                }
                StatusCode::INTERNAL_SERVER_ERROR => println!("{}", "Repository already forked!".red()),
                StatusCode::FORBIDDEN => println!("{}", "Repository fork forbidden!".red()),
                StatusCode::UNPROCESSABLE_ENTITY => println!("{}", "Repository fork input validation failed!".red()),
                StatusCode::NOT_FOUND => println!("{}", "Repository not found!"),
                _ => println!("Repository creation failed! HTTP status code: {}", repo.status().as_str()),
            },
            Err(e) => panic!("{}", e),
        }
    }

    pub fn search_repo(&self, request: &Request) {
        let client = &request.client;
        let arg_value = request
            .arg_value
            .subcommand()
            .1
            .unwrap()
            .value_of("search")
            .unwrap();
        let url = format!(
            "{request}/repos/search?q={query}&token={api_token}",
            request = request.url.as_ref().unwrap(),
            query = arg_value,
            api_token = request.authentication.credentials.1.as_ref().unwrap(),
        );

        let response = client.get(url.as_str()).send();

        match response {
            Ok(repo) => match repo.status() {
                StatusCode::OK => {
                    let deserialized: MultipleRepositories = repo.json().unwrap();

                    match deserialized.data.len() != 0 {
                        true => {
                            println!("{}", "List of repositories found:");

                            for (i, data) in deserialized.data.iter().enumerate() {
                                println!("{}.\tRepository name: {:1}\n\tRepository owner: {:2}\n\tRepository description: {:3}\n", i + 1, data.name, data.owner.as_ref().unwrap().full_name, data.description)
                            }

                            println!("Total number of repositories indexed: {}", deserialized.data.iter().count())
                        }
                        false => println!("{}", "Repository searched doesn't exist!".red()),
                    }
                }
                StatusCode::NOT_FOUND => println!("{}", "Repository searched doesn't exist!".red()),
                StatusCode::UNPROCESSABLE_ENTITY => println!("{}", "Repository input validation failed!".red()),
                _ => println!("Repository search failed! HTTP status code: {}", repo.status().as_str()),
            },
            Err(e) => panic!("{}", e),
        }
    }

    pub fn list_repo(&self, request: &Request) {
        let client = &request.client;
        let url = format!(
            "{request}/repos/search?token={api_token}",
            request = request.url.as_ref().unwrap(),
            api_token = request.authentication.credentials.1.as_ref().unwrap()
        );

        let response = client.get(url.as_str()).send();

        match response {
            Ok(repo) => match repo.status() {
                StatusCode::OK => {
                    let deserialized: MultipleRepositories = repo.json().unwrap();

                    match deserialized.data.len() != 0 {
                                true => {
                                    println!("{}", "List of repositories found:");

                                    for (i, data) in deserialized.data.iter().enumerate() {
                                        println!("{}.\tRepository name: {:1}\n\tRepository owner: {:2}\n\tRepository description: {:3}\n", i + 1, data.name, data.owner.as_ref().unwrap().full_name, data.description)
                                    }

                                    println!("Total number of repositories indexed: {}", deserialized.data.iter().count())
                                },
                                false => println!("{}", "The authenticated user doesn't have any repositories. Why not create one?".yellow())
                            }
                }
                StatusCode::UNPROCESSABLE_ENTITY => println!("{}", "Repository input validation failed!".red()),
                _ => println!("Repository search failed! HTTP status code: {}",repo.status().as_str()),
            },
            Err(e) => panic!("{}", e),
        }
    }

    pub fn push_to_remote(&self, request: &Request) {

    }

    pub fn pull_from_remote(&self, request: &Request) {

    }

    pub fn clone_from_remote(&self, request: &Request, config: &Configuration) {
        let arg_value: Vec<&str> = request
            .arg_value
            .subcommand()
            .1
            .unwrap()
            .values_of("clone")
            .unwrap()
            .collect();
        let url = format!(
            "{base}/{owner}/{repo}",
            base = config.base_url,
            owner = arg_value[0],
            repo = arg_value[1]
        );
        Repo::clone(url.as_str(), ".").unwrap();
        println!("Repository successfully cloned!");
    }
}
