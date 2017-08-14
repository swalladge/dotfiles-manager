use std::env;
use std::path::PathBuf;
use std::fs;

use clap;

pub enum Command {
    Install,
    Uninstall,
    Add,
    Empty,
}

pub struct Args {
    pub dir: PathBuf,
    pub target_dir: PathBuf,
    pub hostname: String,
    pub test: bool,
    pub force: bool,
    pub verbose: bool,
    pub packages: Vec<String>,
    pub command: Command,
}

pub fn get_args(app: clap::App) -> Args {
    let matches = app.get_matches();

    Args {
        dir: match matches.value_of("dir") {
            Some(path) => {
                match fs::canonicalize(path) {
                    Ok(path) => path,
                    Err(_) => {
                        panic!("Invalid 'dir' path");
                    }
                }
            }
            None => env::current_dir().unwrap(),
        },

        target_dir: match matches.value_of("target") {
            Some(path) => {
                match fs::canonicalize(path) {
                    Ok(path) => path,
                    Err(_) => {
                        panic!("Invalid 'dir' path");
                    }
                }
            }
            None => {
                match env::home_dir() {
                    Some(path) => PathBuf::from(path),
                    None => env::current_dir().unwrap(),
                }
            }
        },

        // get the packages list for the command
        packages: match matches.subcommand_name() {
            Some(m) => {
                match matches.subcommand_matches(m) {
                    Some(m2) => {
                        let mut vec = Vec::new();
                        vec.extend(m2.values_of("PACKAGE").unwrap().map(|x| x.to_owned()));
                        vec
                    }
                    _ => vec![],
                }
            }
            _ => vec![],
        },

        // get the packages list for the command
        command: match matches.subcommand_name() {
            Some("install") => Command::Install,
            Some("uninstall") => Command::Uninstall,
            Some("add") => Command::Add,
            _ => Command::Empty,
        },

        force: matches.is_present("force"),

        verbose: matches.is_present("verbose"),

        hostname: match matches.value_of("hostname") {
            Some(name) => name.to_owned(),
            _ => "TEMPHOSTNAME".to_owned(),
        },

        test: matches.is_present("test"),
    }
}
