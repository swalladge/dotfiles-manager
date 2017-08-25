use std::path::{Path, PathBuf};
use std::fs;

pub enum Mode {
    Real,
    Succeed,
    Fail,
}

pub struct FS {
    mode: Mode,
}

impl FS {
    pub fn new() -> FS {
        FS { mode: Mode::Real }
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    pub fn create_link(&self, link: &PathBuf, target: &PathBuf, force: bool) -> bool {
        match self.mode {
            Mode::Succeed => true,
            Mode::Fail => false,
            Mode::Real => {
                // TODO: work on windows too
                use std::os::unix::fs::symlink;
                if force {
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
                    Ok(_) => true,
                    Err(msg) => {
                        println!("failed to create link {:?} | {}", link, msg);
                        false
                    }
                }
            }
        }
    }

    pub fn link_exists<P: AsRef<Path>, Q: AsRef<Path>>(&self, link: P, target: Q) -> bool {
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

    pub fn file_exists<P: AsRef<Path>>(&self, file: P) -> bool {
        match self.mode {
            Mode::Succeed => true,
            Mode::Fail => false,
            Mode::Real => file.as_ref().is_file(),
        }
    }

    pub fn dir_exists<P: AsRef<Path>>(&self, dir: P) -> bool {
        match self.mode {
            Mode::Succeed => true,
            Mode::Fail => false,
            Mode::Real => dir.as_ref().is_dir(),
        }
    }
}
