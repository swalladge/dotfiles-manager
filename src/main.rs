use std::path::{Path, PathBuf};
use std::env;

fn main() {
    println!("Hello, world!");
    let mut f: FS = FS::new();
    f.set_mode(Mode::Real);

    let home: PathBuf = match env::home_dir() {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from(""),
    };

    let result = f.link_exists(home.join(".vimrc"), home.join("dotfiles/vimrc"));
    if result {
        println!("link exists");
    } else {
        println!("link does not exist");
    }

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
            Mode::Real => {
                file.as_ref().is_file()
            }
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
