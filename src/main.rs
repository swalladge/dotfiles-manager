extern crate clap;

use std::path::{Path, PathBuf};
use std::env;
use clap::{Arg, App, SubCommand};
use std::fs;

use args::Args;

mod args;

fn main() {
    let matches = App::new("Dotfiles manager")
        .version("0.0.1")
        .author("Samuel Walladge <samuel@swalladge.id.au>")
        .about("Manages dotfiles")
        .arg(
            Arg::with_name("dir")
                .short("d")
                .long("dir")
                .value_name("DIR")
                .help("Set dir to DIR (default is current dir)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("hostname")
                .short("B")
                .long("hostname")
                .value_name("NAME")
                .help("override computer's hostname to NAME")
                .takes_value(true),
        )
        // TODO: config file options
        .arg(
            Arg::with_name("target")
                .short("t")
                .long("target")
                .value_name("DIR")
                .help("Set target to DIR (default is $HOME)")
                .takes_value(true),
        )
        .arg(Arg::with_name("test").long("no").short("n").help(
            "Do not actually make any filesystem changes or run hooks",
        ))
        .arg(Arg::with_name("verbose").long("verbose").short("v").help(
            "Be verbose",
        ))
        .subcommand(
            SubCommand::with_name("install")
                .about("install tags/packages")
                .arg(
                    Arg::with_name("PACKAGE")
                        .help("package name(s)")
                        .required(true)
                        .multiple(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("remove")
                .about("remove tags/packages")
                .arg(
                    Arg::with_name("PACKAGE")
                        .help("package name(s)")
                        .required(true)
                        .multiple(true),
                )
                .alias("uninstall"),
        )
        .subcommand(
            SubCommand::with_name("add")
                .about("add a file to package")
                .arg(
                    Arg::with_name("file")
                        .help("dotfile to add/adopt")
                        .required(true),
                )
                .arg(Arg::with_name("host").short("b").long("host").help(
                    "add as host-specific",
                ))
                .arg(Arg::with_name("package").short("p").long("package").help(
                    "package name to install to",
                )),
        )
        .get_matches();

    // Gets a value for directory if given - defaults to cwd
    let dir: PathBuf = match matches.value_of("dir") {
        Some(path) => {
            match fs::canonicalize(path) {
                Ok(path) => path,
                Err(_) => {
                    panic!("Invalid 'dir' path");
                }
            }
        }
        None => env::current_dir().unwrap(),
    };

    // Gets a value for directory if given - defaults to cwd
    let target_dir: PathBuf = match matches.value_of("target") {
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

    };


    // let args: Args = Args { dir: PathBuf::from(dir) };

    // println!(
    //     "Value for dir: {}",
    //     fs::canonicalize(args.dir).unwrap().display()
    // );


    // get the packages list for the command
    let mut packages: clap::Values = match matches.subcommand_name() {
        Some(m) => {
            match matches.subcommand_matches(m) {
                Some(m2) => m2.values_of("PACKAGE").unwrap(),
                _ => clap::Values::default(),
            }
        }
        _ => clap::Values::default(),
    };

    // for package in packages {
    //     println!("{}", package);
    // }

    // now we have:
    // - dir: source directory
    // - target_dir: target
    // - packages: iterable of packages to install


    let mut f: FS = FS::new();
    f.set_mode(Mode::Real);

    let result = f.link_exists(target_dir.join(".vimrc"), target_dir.join("dotfiles/vimrc"));
    if result {
        println!("link exists");
    } else {
        println!("link does not exist");
    }

    let package1 = match packages.next() {
        Some(package) => package,
        _ => panic!("no packages"),
    };

    // lets try symlinking things!
    let mut files_base = dir;
    files_base.push(package1);
    files_base.push("files");


    // build a list of directories in dir to be created under target_dir
    // build a list of files to symlink
    // create the dirs
    // symlink the files


    // create all the directories required
    let dirs = get_dirs_to_create(&files_base);
    for dir in dirs {
        let base = dir.strip_prefix(&files_base).unwrap();
        let new_dir = target_dir.join(base);

        let result = fs::create_dir_all(new_dir);
        match result {
            Ok(_) => println!("created ok!"),
            Err(msg) => println!("fail: {}", msg),
        }

    }

    let files = get_files_to_symlink(&files_base);
}

fn get_files_to_symlink(base: &PathBuf) -> Vec<PathBuf> {
    let mut vec = Vec::new();

    for entry in base.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            if entry.file_type().unwrap().is_dir() {
                for file in get_files_to_symlink(&entry.path()) {
                    vec.push(file);
                }
            } else {
                vec.push(entry.path());
            }
        }
    }

    vec
}


fn get_dirs_to_create(base: &PathBuf) -> Vec<PathBuf> {
    let mut vec = Vec::new();

    for entry in base.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            if entry.file_type().unwrap().is_dir() {
                for dir in get_dirs_to_create(&entry.path()) {
                    vec.push(dir);
                }
                vec.push(entry.path());
            }
        }
    }

    vec
}

enum Mode {
    Real,
    Succeed,
    Fail,
}

struct FS {
    mode: Mode,
}

impl FS {
    fn new() -> FS {
        FS { mode: Mode::Real }
    }

    fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    fn create_link<P: AsRef<Path>, Q: AsRef<Path>>(&self, link: P, target: Q) -> bool {
        match self.mode {
            Mode::Succeed => true,
            Mode::Fail => false,
            Mode::Real => {
                use std::os::unix::fs::symlink;
                let result = symlink(link, target);
                match result {
                    Ok(_) => true,
                    Err(_) => false,
                }
            }
        }
    }

    fn link_exists<P: AsRef<Path>, Q: AsRef<Path>>(&self, link: P, target: Q) -> bool {
        match self.mode {
            Mode::Succeed => true,
            Mode::Fail => false,
            Mode::Real => {
                use std::fs;

                let link = fs::read_link(link);
                match link {
                    Ok(link) => link == target.as_ref(),
                    Err(_) => false,
                }
            }
        }
    }

    fn file_exists<P: AsRef<Path>>(&self, file: P) -> bool {
        match self.mode {
            Mode::Succeed => true,
            Mode::Fail => false,
            Mode::Real => file.as_ref().is_file(),
        }
    }
}


#[cfg(test)]
mod tests {
    use Mode;
    use FS;

    #[test]
    fn dummy_fs_works() {
        let mut f: FS = FS::new();
        f.set_mode(Mode::Succeed);
        assert!(f.link_exists("/", "/home"));
        assert!(f.create_link("/", "/home"));
        assert!(f.file_exists("/"));
    }
}
