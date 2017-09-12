use std::path::{Path, PathBuf};
use std::fs;

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

    pub fn link_exists<P: AsRef<Path>, Q: AsRef<Path>>(&self, link: P, target: Q) -> bool {
        use std::fs;

        let link = fs::read_link(link);
        return match link {
            Ok(link) => link == target.as_ref(),
            Err(_) => false,
        };
    }

    pub fn file_exists<P: AsRef<Path>>(&self, file: P) -> bool {
        return file.as_ref().is_file();
    }

    pub fn dir_exists<P: AsRef<Path>>(&self, dir: P) -> bool {
        return dir.as_ref().is_dir();
    }
}
