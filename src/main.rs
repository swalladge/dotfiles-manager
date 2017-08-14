extern crate clap;

use std::path::{Path, PathBuf};
use std::env;
use std::fs;

mod app;
mod args;

fn main() {
    let app = app::new();
    let args = args::get_args(app);


    let mut f: FS = FS::new();
    f.set_mode(Mode::Real);

    // let package1 = match args.packages.next() {
    //     Some(package) => package,
    //     _ => panic!("no packages"),
    // };
    let package1 = "vim";

    // lets try symlinking things!
    let mut files_base = args.dir;
    files_base.push(package1);
    files_base.push("files");


    // create all the directories required
    let dirs = get_dirs_to_create(&files_base);
    for dir in dirs {
        let base = dir.strip_prefix(&files_base).unwrap();
        let new_dir = args.target_dir.join(base);

        let result = fs::create_dir_all(new_dir);
        match result {
            Ok(_) => println!("created ok!"),
            Err(msg) => println!("fail: {}", msg),
        }

    }

    // symlink the files
    let files = get_files_to_symlink(&files_base);
    for file in files {
        let dest = args.target_dir.join(
            file.strip_prefix(&files_base).unwrap(),
        );
        let ok = f.create_link(&file, &dest);
        // TODO: check if worked
    }

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

    fn create_link(&self, link: &PathBuf, target: &PathBuf) -> bool {
        match self.mode {
            Mode::Succeed => true,
            Mode::Fail => false,
            Mode::Real => {
                // TODO: work on windows too
                use std::os::unix::fs::symlink;
                let result = symlink(link, target);
                match result {
                    Ok(_) => true,
                    Err(msg) => {
                        println!("failed to create link {:?} | {}", link, msg);
                        false
                    }
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
