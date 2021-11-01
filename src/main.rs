use clap::{crate_authors, crate_name, crate_version, App, Arg, SubCommand};
use git2::Repository;
use log::*;
use std::env::current_dir;
use std::path::PathBuf;
use std::process::exit;

fn main() {
    let app = App::new(crate_name!())
        .author(crate_authors!())
        .version(crate_version!())
        .arg(
            Arg::with_name("cwd")
                .short("C")
                .value_name("path")
                .takes_value(true)
                .help("Sets the working directory"),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .arg(
            Arg::with_name("quiet")
                .short("q")
                .long("quiet")
                .help("Silences all output"),
        )
        .subcommand(
            SubCommand::with_name("add")
                .arg(Arg::with_name("ref").index(1))
                .help("Add a ref to ipfs"),
        );

    let matches = app.get_matches();

    let wd = matches
        .value_of("cwd")
        .and_then(|s| Some(PathBuf::from(s)))
        .unwrap_or(current_dir().unwrap());

    stderrlog::new()
        .module(module_path!())
        .quiet(matches.is_present("quiet"))
        .verbosity(matches.occurrences_of("verbose") as usize + 1)
        .timestamp(stderrlog::Timestamp::Off)
        .init()
        .unwrap();

    info!("Working Directory: {}", wd.display());

    let repo = match Repository::discover(wd.clone()) {
        Ok(repo) => repo,
        Err(e) => {
            error!("failed to init git repo: {}", e);
            exit(1);
        }
    };

    match matches.subcommand() {
        ("add", Some(add_matches)) => {
            let r = match add_matches.value_of("ref")
            .and_then(|s| Some(repo.find_reference(s)))
            .unwrap_or(repo.head()) {
                Ok(r) => r,
                Err(e) => {
                    error!("{}", e);
                    exit(1);
                }
            };
            info!("Adding {}", r.name().unwrap());
        }
        _ => {
            println!("{}", matches.usage());
        }
    }
}
