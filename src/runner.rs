use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashMap;
use std::process::Command;

use args::Args;


pub struct Runner<'a> {
    args: &'a Args,
}

impl<'a> Runner<'a> {
    pub fn new(args: &Args) -> Runner {
        Runner { args: args }
    }

    pub fn install(&self) -> bool {

        let args = self.args;

        let mut f: FS = FS::new();
        f.set_mode(Mode::Real);

        // let package1 = match args.packages.next() {
        //     Some(package) => package,
        //     _ => panic!("no packages"),
        // };
        let package1 = "vim";

        // lets try symlinking things!
        let mut package_base = args.dir.clone();
        package_base.push(package1);

        let mut global_hooks_base = package_base.clone();
        global_hooks_base.push("hooks");

        let mut global_files_base = package_base.clone();

        global_files_base.push("files");

        // create all the directories required
        let dirs = get_dirs_to_create(&global_files_base);
        for dir in dirs {
            let base = dir.strip_prefix(&global_files_base).unwrap();
            let new_dir = args.target_dir.join(base);

            let result = fs::create_dir_all(new_dir);
            match result {
                Ok(_) => println!("created ok!"),
                Err(msg) => println!("fail: {}", msg),
            }

        }

        // host specific config
        let mut host_files_base = package_base.clone();
        host_files_base.push(&args.hostname);
        host_files_base.push("files");

        let mut host_files: Vec<PathBuf> = vec![];

        if f.dir_exists(&host_files_base) {

            let host_dirs = get_dirs_to_create(&host_files_base);
            for dir in host_dirs {
                let base = dir.strip_prefix(&global_files_base).unwrap();
                let new_dir = args.target_dir.join(base);

                let result = fs::create_dir_all(new_dir);
                match result {
                    Ok(_) => println!("created ok!"),
                    Err(msg) => println!("fail: {}", msg),
                }

            }

            // symlink the files
            host_files = get_files_to_symlink(&host_files_base);
        }

        let files = get_files_to_symlink(&global_files_base);

        // map destinations to link targets
        // this method allows host-specfic files to take precedence
        let mut dests: HashMap<PathBuf, PathBuf> = HashMap::new();
        for file in host_files {
            let dest = args.target_dir.join(
                file.strip_prefix(&host_files_base).unwrap(),
            );
            dests.insert(dest, file.clone());
        }

        for file in files {
            let dest = args.target_dir.join(
                file.strip_prefix(&global_files_base)
                    .unwrap(),
            );
            if !dests.contains_key(&dest) {
                dests.insert(dest, file.clone());
            }
        }

        for (dest, file) in dests {
            // dest is the new file to be created
            // it should be a symbolic link pointing to file
            let ok = f.create_link(&dest, &file, args.force);
            // TODO: check if worked
        }



        // Now for the post-up hooks!


        let mut post_up_hooks_dir = global_hooks_base.clone();
        post_up_hooks_dir.push("post-up");

        let mut hooks_files = Vec::new();

        for entry in post_up_hooks_dir.read_dir().expect(
            "read_dir call failed (post-up hooks dir) - no post-up hooks will run",
        )
        {
            if let Ok(entry) = entry {
                if entry.file_type().unwrap().is_file() {
                    hooks_files.push(entry.path());
                }
            }
        }

        for path in hooks_files {
            let s = path.as_os_str();
            println!(
                "Running hook {:?}",
                path.strip_prefix(&package_base).unwrap()
            );

            Command::new(s).spawn().expect(
                "hook exited with fail status",
            );
        }


        return true;
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

    fn create_link(&self, link: &PathBuf, target: &PathBuf, force: bool) -> bool {
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
                                fs::remove_file(&link);
                            } else if link.is_dir() {
                                fs::remove_dir_all(&link);
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

    fn dir_exists<P: AsRef<Path>>(&self, dir: P) -> bool {
        match self.mode {
            Mode::Succeed => true,
            Mode::Fail => false,
            Mode::Real => dir.as_ref().is_dir(),
        }
    }
}
