use std::path::PathBuf;
use std::fs;
use std::collections::{HashMap, HashSet};

use args::{Args, AddArgs};
use hooks;
use file_ops::{FS, Mode};


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

        for package1 in &args.packages {

            let mut package_base = args.dir.clone();
            package_base.push(package1);

            let mut global_hooks_base = package_base.clone();
            global_hooks_base.push("hooks");

            // run the pre-up hooks

            let mut pre_up_hooks_dir = global_hooks_base.clone();
            pre_up_hooks_dir.push("pre-up");
            let mut host_pre_up_hooks_dir = package_base.clone();
            host_pre_up_hooks_dir.push("hosts");
            host_pre_up_hooks_dir.push(&args.hostname);
            host_pre_up_hooks_dir.push("hooks");
            host_pre_up_hooks_dir.push("pre-up");

            hooks::run_hooks(&pre_up_hooks_dir, &host_pre_up_hooks_dir);


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
            host_files_base.push("hosts");
            host_files_base.push(&args.hostname);
            host_files_base.push("files");

            let mut host_files: Vec<PathBuf> = vec![];

            if f.dir_exists(&host_files_base) {

                let host_dirs = get_dirs_to_create(&host_files_base);
                for dir in host_dirs {
                    let base = dir.strip_prefix(&host_files_base).unwrap();
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
                if !ok {
                    return false;
                }
            }



            // Now for the post-up hooks!

            let mut post_up_hooks_dir = global_hooks_base.clone();
            post_up_hooks_dir.push("post-up");
            let mut host_post_up_hooks_dir = package_base.clone();
            host_post_up_hooks_dir.push("hosts");
            host_post_up_hooks_dir.push(&args.hostname);
            host_post_up_hooks_dir.push("hooks");
            host_post_up_hooks_dir.push("post-up");

            hooks::run_hooks(&post_up_hooks_dir, &host_post_up_hooks_dir);

        }

        return true;
    }

    pub fn uninstall(&self) -> bool {

        let args = self.args;

        let mut f: FS = FS::new();
        f.set_mode(Mode::Real);

        for package1 in &args.packages {

            let mut package_base = args.dir.clone();
            package_base.push(package1);

            let mut global_hooks_base = package_base.clone();
            global_hooks_base.push("hooks");


            // run the pre-down hooks

            let mut pre_down_hooks_dir = global_hooks_base.clone();
            pre_down_hooks_dir.push("pre-down");
            let mut host_pre_down_hooks_dir = package_base.clone();
            host_pre_down_hooks_dir.push(format!("hosts/{}/hooks/pre-down/", &args.hostname));
            println!("{:?}", host_pre_down_hooks_dir);

            hooks::run_hooks(&pre_down_hooks_dir, &host_pre_down_hooks_dir);


            let mut global_files_base = package_base.clone();
            global_files_base.push("files");

            // host specific config
            let mut host_files_base = package_base.clone();
            host_files_base.push(format!("hosts/{}/files/", &args.hostname));

            let mut host_files: Vec<PathBuf> = vec![];

            if f.dir_exists(&host_files_base) {

                let host_dirs = get_dirs_to_create(&host_files_base);
                for dir in host_dirs {
                    let base = dir.strip_prefix(&host_files_base).unwrap();
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
            let mut dests: HashSet<PathBuf> = HashSet::new();
            for file in host_files {
                let dest = args.target_dir.join(
                    file.strip_prefix(&host_files_base).unwrap(),
                );
                dests.insert(dest);
            }

            for file in files {
                let dest = args.target_dir.join(
                    file.strip_prefix(&global_files_base)
                        .unwrap(),
                );
                if !dests.contains(&dest) {
                    dests.insert(dest);
                }
            }

            for dest in dests {
                // dest is the new file to be created
                // it should be a symbolic link pointing to file

                // if the file doesn't exist, then don't do anything
                if !dest.exists() {
                    continue;
                }

                // check if we should remove it
                // resolve the symlinks and check where it points, and whether force is set
                match dest.canonicalize() {
                    Ok(path) => {
                        if !path.starts_with(&package_base) {
                            if !args.force {
                                println!(
                                    "{:?} exists and not pointing to the package base ({})",
                                    &dest,
                                    &package1
                                );
                                continue;
                            }
                        }
                    }
                    Err(msg) => {
                        println!("Error checking existing file {:?} : {}", &dest, msg);
                    }
                }

                // delete!
                let res;
                if dest.is_dir() {
                    res = fs::remove_dir_all(&dest);
                } else {
                    res = fs::remove_file(&dest);
                }
                match res {
                    Ok(_) => {
                        println!("Deleted {:?}", &dest);
                    }
                    Err(msg) => {
                        println!("Failed to remove {:?} : {}", &dest, msg);
                    }
                }

            }


            // Now for the post-down hooks!

            let mut post_down_hooks_dir = global_hooks_base.clone();
            post_down_hooks_dir.push("post-down");
            let mut host_post_down_hooks_dir = package_base.clone();
            host_post_down_hooks_dir.push("hosts");
            host_post_down_hooks_dir.push(&args.hostname);
            host_post_down_hooks_dir.push("hooks");
            host_post_down_hooks_dir.push("post-down");

            hooks::run_hooks(&post_down_hooks_dir, &host_post_down_hooks_dir);

        }

        return true;
    }

    pub fn add(&self) -> bool {
        // get the subcommand arguments - guaranteed to be present because this function only
        // called when add subcommand used
        let add_args = match &self.args.add_args {
            &Some(ref args) => args,
            _ => panic!("should never happen"),
        };

        println!("Adding file {:?}", add_args.filename);
        println!("to package {:?}", add_args.package);
        println!(
            "host-specific mode is {}",
            if add_args.host_specific { "on" } else { "off" }
        );

        // TODO: implement

        true
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
