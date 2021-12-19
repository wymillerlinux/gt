use std::collections::HashMap;

use colored::*;
use reqwest::StatusCode;
use serde_derive::{Serialize, Deserialize};
use serde_json::Value;

use crate::request::Request;
use crate::util;

pub struct Issue;

#[derive(Serialize, Deserialize)]
pub struct MultipleIssues {
    pub data: Vec<IssueResponse>,
    pub ok: bool,
}

#[derive(Serialize, Deserialize)]
pub struct IssueResponse {
    pub id: i64,
    pub url: String,
    pub html_url: String,
    pub number: i64,
    pub user: User,
    pub original_author: String,
    pub original_author_id: i64,
    pub title: String,
    pub body: String,
    pub ref_field: String,
    pub labels: Vec<Value>,
    pub milestone: Value,
    pub assignee: Value,
    pub assignees: Value,
    pub state: String,
    pub is_locked: bool,
    pub comments: i64,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: Value,
    pub due_date: Value,
    pub pull_request: Value,
    pub repository: Repository,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub login: String,
    pub full_name: String,
    pub email: String,
    pub avatar_url: String,
    pub language: String,
    pub is_admin: bool,
    pub last_login: String,
    pub created: String,
    pub restricted: bool,
    pub username: String,
}

#[derive(Serialize, Deserialize)]
pub struct Repository {
    pub id: i64,
    pub name: String,
    pub owner: String,
    pub full_name: String,
}


impl Issue {
    pub fn new() -> Issue {
        Issue {}
    }

    pub fn create_issue(&self, request: &Request) {
        let issue_title: String;
        let issue_description: String;
        let _issue_assignee: Option<String>;
        let _issue_milestone: Option<String>;
        let _issue_tags: Option<String>;
        
        let client = &request.client;
        let arg_value: Vec<&str> = request
            .arg_value
            .subcommand()
            .1
            .unwrap()
            .values_of("create")
            .unwrap()
            .collect();
        let mut map: HashMap<&str, String> = HashMap::new();
        let url = format!(
            "{request}/repos/{owner}/{repo}/issues?token={api_token}",
            request = request.url.as_ref().unwrap(),
            owner = arg_value[0],
            repo = arg_value[1],
            api_token = request.authentication.credentials.1.as_ref().unwrap()
        );

        //let issue_title = read!("Enter a title for the issue: {}\n");
        //let issue_description = read!("Enter a description for the issue: {}\n");
        issue_title = util::get_input("Enter a title for the issue: ".to_string());
        issue_description = util::get_input("Enter a description for the issue: ".to_string());
        map.insert("title", issue_title);
        map.insert("body", issue_description);

        let response = client.post(&url).json(&map).send();

        match response {
            Ok(repo) => {
                match repo.status() {
                    StatusCode::CREATED => println!("{}", "Issue successfully created!".green()),
                    StatusCode::FORBIDDEN => println!("{}", "Issue creation forbidden!".red()),
                    StatusCode::UNPROCESSABLE_ENTITY => println!("{}", "Issue input validation failed!".red()),
                    _ => println!(),
                }
            },
            Err(e) => panic!("{}", e),
        }
    }
}
