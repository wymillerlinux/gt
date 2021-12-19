// gt - a gitea cli client
// Written by Wyatt J. Miller
// All right reserved, 2020 - 2021
// Licensed by the MPL v2

mod arg;
mod config;
mod issue;
mod repo;
mod request;
mod pr;
mod user;
mod util;

use clap::ArgMatches;

fn main() {
    let matches: ArgMatches = arg::get_args();
    let config = crate::config::Configuration::new();
    let auth = request::Authentication::new(&config);
    let request = auth.request_chooser(config.clone(), matches);

    match request.arg_value.subcommand() {
        ("", None) => println!("No subcommand was given!"),
        ("repo", Some(repo_matches)) => {
            let repo = repo::Repository::new();

            // TODO: match expression should be here
            if repo_matches.is_present("create") {
                repo.create_repo(&request);
            }

            if repo_matches.is_present("delete") {
                repo.delete_repo(&request);
            }

            if repo_matches.is_present("fork") {
                repo.fork_repo(&request)
            }

            if repo_matches.is_present("search") {
                repo.search_repo(&request)
            }

            if repo_matches.is_present("list") {
                repo.list_repo(&request)
            }
        }
        ("issue", Some(issue_matches)) => {
            let issue = issue::Issue::new();

            // TODO: match expression should be here
            if issue_matches.is_present("create") {
                issue.create_issue(&request);
            }
        }
        _ => println!("Huh?"),
    }
}
