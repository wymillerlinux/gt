// gt - a gitea cli client
// Written by Wyatt J. Miller
// All right reserved, 2020
// Licensed by the MPL v2

mod arg;
mod config;
mod issue;
mod repo;

use clap::{ArgMatches};

fn main() {
    let matches: ArgMatches = arg::get_args();
    let config = crate::config::Configuration::new();

    match matches.subcommand() {
        ("", None) => println!("No subcommand was given!"),
        ("repo", Some(repo_matches)) => {
            let repo = repo::Repository::new();

            // TODO: match expression should be here
            if repo_matches.is_present("create") {
                repo.create_repo(&config, repo_matches);
            }

            if repo_matches.is_present("delete") {
                repo.delete_repo(&config, repo_matches);
            }

            if repo_matches.is_present("fork") {
                repo.fork_repo(&config, repo_matches)
            }

            if repo_matches.is_present("search") {
                repo.search_repo(&config, repo_matches)
            }
            
            if repo_matches.is_present("list") {
                repo.list_repo(&config)
            }
        },
        ("issue", Some(issue_matches)) => {
            let issue = issue::Issue::new();

            // TODO: match expression should be here
            if issue_matches.is_present("create") {
                issue.create_issue(&config, issue_matches);
            }
        },
        _ => println!("Huh?")
    }
}