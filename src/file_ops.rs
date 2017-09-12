use std::path::{Path, PathBuf};
use std::fs;
use std::io;

pub struct FS {
    force: bool,
    simulate: bool,
}

impl FS {
    pub fn new(force: bool, simulate: bool) -> FS {
        FS {
            force: force,
            simulate: simulate,
        }
    }

    pub fn create_link(&self, link: &PathBuf, target: &PathBuf) -> bool {
        // TODO: work on windows too
        use std::os::unix::fs::symlink;

        if self.force {
            match fs::canonicalize(&link) {
                Ok(_) => {
                    if link.is_file() {
                        println!("removing {}", link.display());
                        let result = fs::remove_file(&link);
                        match result {
                            Err(msg) => {
                                println!("Failed to remove file: {}", msg);
                                return false;
                            }
                            _ => (),
                        }
                    } else if link.is_dir() {
                        let result = fs::remove_dir_all(&link);
                        match result {
                            Err(msg) => {
                                println!("Failed to remove directory: {}", msg);
                                return false;
                            }
                            _ => (),
                        }
                    }
                }
                _ => (),
            }
        } else {
            match fs::canonicalize(&link) {
                Ok(file) => {
                    if &file == target {
                        println!("Link already exists: {:?}", file);
                        return true;
                    }
                }
                _ => (),
            }
        }

        let result = symlink(target, link);
        match result {
            Ok(_) => return true,
            Err(msg) => {
                println!("failed to create link {:?} | {}", link, msg);
                return false;
            }
        }
    }

    pub fn dir_exists<P: AsRef<Path>>(&self, dir: P) -> bool {
        return dir.as_ref().is_dir();
    }

    pub fn create_dir_all<P: AsRef<Path>>(&self, dir: P) -> io::Result<()> {
        return fs::create_dir_all(dir);
    }

    pub fn remove_dir_all<P: AsRef<Path>>(&self, dir: P) -> io::Result<()> {
        return fs::remove_dir_all(dir);
    }

    pub fn remove_file<P: AsRef<Path>>(&self, dir: P) -> io::Result<()> {
        return fs::remove_file(dir);
    }

    pub fn rename<P: AsRef<Path>, Q: AsRef<Path>>(&self, old: P, new: Q) -> io::Result<()> {
        return fs::rename(old, new);
    }

    pub fn get_files_to_symlink(&self, base: &PathBuf) -> Vec<PathBuf> {
        let mut vec = Vec::new();

        for entry in base.read_dir().expect("read_dir call failed") {
            if let Ok(entry) = entry {
                if entry.file_type().unwrap().is_dir() {
                    for file in self.get_files_to_symlink(&entry.path()) {
                        vec.push(file);
                    }
                } else {
                    vec.push(entry.path());
                }
            }
        }

        vec
    }


    pub fn get_dirs_to_create(&self, base: &PathBuf) -> Vec<PathBuf> {
        let mut vec = Vec::new();

        for entry in base.read_dir().expect("read_dir call failed") {
            if let Ok(entry) = entry {
                if entry.file_type().unwrap().is_dir() {
                    for dir in self.get_dirs_to_create(&entry.path()) {
                        vec.push(dir);
                    }
                    vec.push(entry.path());
                }
            }
        }

        vec
    }

    pub fn exists(&self, path: &PathBuf) -> bool {
        return path.exists();
    }
}
