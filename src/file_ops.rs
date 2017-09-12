use std::path::{Path, PathBuf};
use std::fs;
use std::io;
use std::io::ErrorKind;

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
        // `link` is path to symlink to create
        // `target` is path to file in repo the link should point to
        // TODO: work on windows too
        use std::os::unix::fs::symlink;

        if self.force {
            match fs::canonicalize(&link) {
                Ok(_) => {
                    println!("Deleting existing file: {}", link.display());

                    // don't actually remove a file in simulate mode
                    if self.simulate {
                        if self.is_writable(link) {
                            return true;
                        } else {
                            println!("Failed to delete: file is readonly");
                            return false;
                        }
                    } else {

                        if link.is_file() {

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

        if self.simulate {
            if self.is_writable(link.parent().unwrap()) {
                return true;
            } else {
                println!("No permissions to write in {:?}", link.parent().unwrap());
                return false;
            }
        } else {

            let result = symlink(target, link);
            match result {
                Ok(_) => return true,
                Err(msg) => {
                    println!("failed to create link {:?} | {}", link, msg);
                    return false;
                }
            }
        }
    }

    pub fn dir_exists<P: AsRef<Path>>(&self, dir: P) -> bool {
        return dir.as_ref().is_dir();
    }

    pub fn create_dir_all(&self, dir: &PathBuf) -> io::Result<()> {
        if self.simulate {
            let mut parent = dir.parent().unwrap();
            while !self.dir_exists(parent) {
                match parent.parent() {
                    None => {
                        break;
                    }
                    Some(path) => {
                        parent = path;
                    }
                }
            }

            if self.is_writable(parent) {
                return Ok(());
            } else {
                return Err(io::Error::new(
                    ErrorKind::PermissionDenied,
                    "file not writeable",
                ));
            }
        }
        return fs::create_dir_all(dir);
    }

    pub fn remove_dir_all<P: AsRef<Path>>(&self, dir: P) -> io::Result<()> {
        if self.simulate {
            if self.is_writable(dir) {
                return Ok(());
            } else {
                return Err(io::Error::new(
                    ErrorKind::PermissionDenied,
                    "file not writeable",
                ));
            }
        }
        return fs::remove_dir_all(dir);
    }

    pub fn remove_file<P: AsRef<Path>>(&self, dir: P) -> io::Result<()> {
        if self.simulate {
            if self.is_writable(dir) {
                return Ok(());
            } else {
                return Err(io::Error::new(
                    ErrorKind::PermissionDenied,
                    "file not writeable",
                ));
            }
        }
        return fs::remove_file(dir);
    }

    pub fn rename(&self, old: &PathBuf, new: &PathBuf) -> io::Result<()> {
        if self.simulate {
            if self.is_writable(old) && self.is_writable(new.parent().unwrap()) {
                return Ok(());
            } else {
                return Err(io::Error::new(
                    ErrorKind::PermissionDenied,
                    "file(s) not writeable",
                ));
            }
        }
        return fs::rename(old, new);
    }

    // recursively scans the `base` directory and builds a list of files under that path
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


    // recursively scans the `base` directory and builds a list of directories under that path
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

    pub fn is_writable<P: AsRef<Path>>(&self, path: P) -> bool {
        match fs::metadata(path) {
            Ok(metadata) => !metadata.permissions().readonly(),
            Err(_) => false,
        }
    }
}
