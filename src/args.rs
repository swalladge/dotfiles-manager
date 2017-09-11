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

pub struct AddArgs {
    pub host_specific: bool,
    pub package: String,
    pub filename: PathBuf,
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
    pub add_args: Option<AddArgs>,
}

pub fn get_args(matches: clap::ArgMatches) -> Result<Args, &'static str> {

    let dir = match matches.value_of("dir") {
        Some(path) => {
            match fs::canonicalize(path) {
                Ok(path) => path,
                Err(_) => {
                    return Err("invalid repo dir (-d) path");
                }
            }
        }
        None => env::current_dir().unwrap(),
    };

    let target_dir = match matches.value_of("target") {
        Some(path) => {
            match fs::canonicalize(path) {
                Ok(path) => path,
                Err(_) => {
                    return Err("invalid target dir path");
                }
            }
        }
        None => {
            match env::home_dir() {
                Some(path) => PathBuf::from(path),
                None => {
                    return Err("could not determine home directory");
                }
            }
        }
    };

    let hostname = match matches.value_of("hostname") {
        Some(name) => name.to_owned(),
        _ => {
            match sys_info::hostname() {
                Ok(name) => name,
                Err(_) => {
                    return Err("could not determine hostname");
                }
            }
        }
    };

    // get the args for the add command
    // also currently checks to see if the file exists and isn't a symlink
    let add_args = match matches.subcommand() {
        ("add", Some(m)) => {
            Some(AddArgs {
                host_specific: m.is_present("host"),
                package: m.value_of("package").unwrap().to_owned(),
                filename: {
                    let filename = PathBuf::from(m.value_of("file").unwrap());
                    // don't allow adding a file that is a symlink
                    if filename.read_link().is_ok() {
                        return Err("cannot add symbolic link");
                    }
                    match fs::canonicalize(filename) {
                        Ok(path) => path,
                        Err(msg) => {
                            println!("{}", msg);
                            return Err("Could not add file");
                        }
                    }
                },
            })
        }
        _ => None,

    };

    let args = Args {
        dir: dir,
        target_dir: target_dir,
        force: matches.is_present("force"),
        verbose: matches.is_present("verbose"),
        hostname: hostname,
        test: matches.is_present("test"),

        // get the packages list for the command
        command: match matches.subcommand_name() {
            Some("install") => Command::Install,
            Some("uninstall") => Command::Uninstall,
            Some("remove") => Command::Uninstall,
            Some("add") => Command::Add,
            _ => Command::Empty,
        },

        // get the packages list for the command
        packages: match matches.subcommand() {
            ("install", Some(m)) |
            ("uninstall", Some(m)) |
            ("remove", Some(m)) => {
                let mut vec = Vec::new();
                vec.extend(m.values_of("PACKAGE").unwrap().map(|x| x.to_owned()));
                vec
            }
            _ => vec![],
        },

        add_args: add_args,
    };
    Ok(args)
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs;
    use std::path::PathBuf;

    use args;
    use app;

    #[test]
    fn check_verbose() {
        let app = app::new();
        let app_args = vec!["dotfiles-manager", "-v"];
        let args = args::get_args(app.get_matches_from(app_args)).unwrap();
        assert!(args.verbose);
    }

    #[test]
    fn check_test() {
        let app = app::new();
        let app_args = vec!["dotfiles-manager", "-n"];
        let args = args::get_args(app.get_matches_from(app_args)).unwrap();
        assert!(args.test);
    }

    #[test]
    fn check_test_long_args() {
        let app = app::new();
        let app_args = vec!["dotfiles-manager", "--no"];
        let args = args::get_args(app.get_matches_from(app_args)).unwrap();
        assert!(args.test);
    }

    #[test]
    fn check_hostname_given() {
        let app = app::new();
        let app_args = vec!["dotfiles-manager", "-B", "myhostname", "install", "vim"];
        let args = args::get_args(app.get_matches_from(app_args)).unwrap();
        assert_eq!(args.hostname, "myhostname");
    }


    #[test]
    fn check_hostname_discovered() {
        let app = app::new();
        let app_args = vec!["dotfiles-manager", "install", "vim"];
        let args = args::get_args(app.get_matches_from(app_args)).unwrap();
        // make sure a hostname is found
        assert!(args.hostname.len() > 0);
    }


    #[test]
    fn check_force_on() {
        let app = app::new();
        let app_args = vec!["dotfiles-manager", "-f", "install", "vim"];
        let args = args::get_args(app.get_matches_from(app_args)).unwrap();
        assert!(args.force);
    }

    #[test]
    fn check_force_off() {
        let app = app::new();
        let app_args = vec!["dotfiles-manager", "install", "vim"];
        let args = args::get_args(app.get_matches_from(app_args)).unwrap();
        assert_eq!(args.force, false);
    }

    #[test]
    fn check_find_package_names() {
        let app = app::new();
        let app_args = vec!["dotfiles-manager", "install", "vim", "zsh"];
        let args = args::get_args(app.get_matches_from(app_args)).unwrap();
        assert_eq!(args.packages, vec!["vim", "zsh"]);
    }

    #[test]
    fn check_dir() {
        let app = app::new();
        let app_args = vec![
            "dotfiles-manager",
            "-d",
            "test/home",
            "install",
            "vim",
            "zsh",
        ];
        let args = args::get_args(app.get_matches_from(app_args)).unwrap();
    }

    #[test]
    fn check_invalid_dir() {
        let app = app::new();
        let app_args = vec![
            "dotfiles-manager",
            "-d",
            "test/doesnotexist",
            "install",
            "vim",
            "zsh",
        ];
        let args = args::get_args(app.get_matches_from(app_args));
        assert!(args.is_err(), "should be Err because dir doesn't exist");
    }

    #[test]
    fn check_invalid_target_dir() {
        let app = app::new();
        let app_args = vec![
            "dotfiles-manager",
            "-t",
            "test/doesnotexist",
            "install",
            "vim",
            "zsh",
        ];
        let args = args::get_args(app.get_matches_from(app_args));
        assert!(args.is_err(), "should be Err because dir doesn't exist");
    }

    #[test]
    fn check_default_target_dir() {
        let app = app::new();
        let app_args = vec!["dotfiles-manager", "install", "vim", "zsh"];
        let args = args::get_args(app.get_matches_from(app_args)).unwrap();
        assert_eq!(args.target_dir, env::home_dir().unwrap());
    }

    #[test]
    fn check_given_target_dir() {
        let app = app::new();
        let target_dir = fs::canonicalize("test/home").unwrap();
        let app_args = vec![
            "dotfiles-manager",
            "--target",
            target_dir.to_str().unwrap(),
            "install",
            "vim",
            "zsh",
        ];
        let args = args::get_args(app.get_matches_from(app_args)).unwrap();
        assert_eq!(args.target_dir, PathBuf::from(&target_dir));
    }


    #[test]
    fn check_add_file_args() {
        let app = app::new();
        let target_dir = fs::canonicalize("test/home").unwrap();
        let file = fs::canonicalize("test/repo/vim/files/.vimrc").unwrap();
        let app_args = vec![
            "dotfiles-manager",
            "--target",
            target_dir.to_str().unwrap(),
            "add",
            "test/repo/vim/files/.vimrc",
            "--host",
            "--package",
            "zsh",
        ];
        let args = args::get_args(app.get_matches_from(app_args)).unwrap();
        let add_args = args.add_args.unwrap();
        assert_eq!((&add_args).host_specific, true);
        assert_eq!((&add_args).filename, file);
        assert_eq!((&add_args).package, "zsh");
    }


    #[test]
    fn check_add_file_no_package() {
        let app = app::new();
        let target_dir = fs::canonicalize("test/home").unwrap();
        let file = fs::canonicalize("test/repo/vim/files/.vimrc").unwrap();
        let app_args = vec![
            "dotfiles-manager",
            "--target",
            target_dir.to_str().unwrap(),
            "add",
            "test/repo/vim/files/.vimrc",
            "--host",
        ];
        let matches = app.get_matches_from_safe(app_args);
        assert!(
            matches.is_err(),
            "should be Err because package name not given"
        );
    }

    #[test]
    fn check_add_file_no_host() {
        let app = app::new();
        let target_dir = fs::canonicalize("test/home").unwrap();
        let file = fs::canonicalize("test/repo/vim/files/.vimrc").unwrap();
        let app_args = vec![
            "dotfiles-manager",
            "--target",
            target_dir.to_str().unwrap(),
            "add",
            "test/repo/vim/files/.vimrc",
            "--package",
            "zsh",
        ];
        let args = args::get_args(app.get_matches_from(app_args)).unwrap();
        let add_args = args.add_args.unwrap();
        assert_eq!((&add_args).host_specific, false);
    }

}
