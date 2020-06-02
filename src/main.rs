// gt - a gitea cli client
// Written by Wyatt J. Miller
// All right reserved, 2020

use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("gt - A Gitea CLI client")
                                .version("0.0.1")
                                .author("Wyatt J. Miller <wjmiller2016@gmail.com>")
                                .about("It's a CLI client, what do you expect?")
                                .subcommand(SubCommand::with_name("repo")
                                        .about("Create, delete, or fork a repo")
                                        .arg(Arg::with_name("create")
                                                .short("c")
                                                .long("create")
                                                .value_names(&["OWNER", "REPO"])
                                                .help("Create a repo"))
                                        .arg(Arg::with_name("delete")
                                                .short("d")
                                                .long("delete")
                                                .value_names(&["OWNER", "REPO"])
                                                .help("Delete a repo"))
                                        .arg(Arg::with_name("fork")
                                                .short("f")
                                                .long("fork")
                                                .value_names(&["OWNER", "REPO", "FORKED_OWNER", "FORKED_REPO"])
                                                .help("Fork a repo")))
                                .get_matches();

    match matches.subcommand() {
        ("repo", Some(repo_matches)) => {
            if repo_matches.is_present("create") {
                println!("\"repo create\" passed")
            }

            if repo_matches.is_present("delete") {
                println!("\"repo delete\" passed")
            }

            if repo_matches.is_present("fork") {
                println!("\"repo fork\" passed")
            }
        }
        ("", None) => println!("No subcommand was given!"),
        _ => unreachable!()
    }
}
