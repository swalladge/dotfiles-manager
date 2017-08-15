use std::env;
use std::path::PathBuf;
use std::fs;

use clap;
use sys_info;

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

pub fn get_args(matches: clap::ArgMatches) -> Args {

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
            _ => {
                match sys_info::hostname() {
                    Ok(name) => name,
                    Err(_) => {
                        println!("Hostname discovery failed, disabling host specific tasks!");
                        "".to_owned()
                    }
                }
            }
        },

        test: matches.is_present("test"),
    }
}

#[cfg(test)]
mod tests {
    use args;
    use app;

    #[test]
    fn check_verbose() {
        let app = app::new();
        let app_args = vec!["dotfiles-manager", "-v"];
        let args = args::get_args(app.get_matches_from(app_args));
        assert!(args.verbose);
    }

    #[test]
    fn check_test() {
        let app = app::new();
        let app_args = vec!["dotfiles-manager", "-n"];
        let args = args::get_args(app.get_matches_from(app_args));
        assert!(args.test);
    }

    #[test]
    fn check_test_long_args() {
        let app = app::new();
        let app_args = vec!["dotfiles-manager", "--no"];
        let args = args::get_args(app.get_matches_from(app_args));
        assert!(args.test);
    }

    #[test]
    fn check_hostname_given() {
        let app = app::new();
        let app_args = vec!["dotfiles-manager", "-B", "myhostname", "install", "vim"];
        let args = args::get_args(app.get_matches_from(app_args));
        assert_eq!(args.hostname, "myhostname");
    }


    #[test]
    fn check_hostname_discovered() {
        let app = app::new();
        let app_args = vec!["dotfiles-manager", "install", "vim"];
        let args = args::get_args(app.get_matches_from(app_args));
        // make sure a hostname is found
        assert!(args.hostname.len() > 0);
    }


    #[test]
    fn check_force_on() {
        let app = app::new();
        let app_args = vec!["dotfiles-manager", "-f", "install", "vim"];
        let args = args::get_args(app.get_matches_from(app_args));
        assert!(args.force);
    }

    #[test]
    fn check_force_off() {
        let app = app::new();
        let app_args = vec!["dotfiles-manager", "install", "vim"];
        let args = args::get_args(app.get_matches_from(app_args));
        assert_eq!(args.force, false);
    }

}
