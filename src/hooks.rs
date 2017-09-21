use std::path::PathBuf;
use std::process::Command;
use std::collections::HashMap;
use std::ffi::OsString;

pub fn run_hooks(dir: &PathBuf, host_dir: &PathBuf, simulate: bool) -> bool {

    let mut hooks_files = HashMap::new();

    // collect the hooks from the main dir
    match dir.read_dir() {
        Ok(dirs) => {
            for entry in dirs {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        if entry.file_type().unwrap().is_file() {
                            hooks_files.insert(path.file_name().unwrap().to_os_string(), path);
                        }
                    }
                    Err(msg) => {
                        println!("{}", msg);
                    }
                }
            }
        }
        Err(msg) => {
            println!("{:?} {}", dir, msg);
        }
    }

    // collect the host-specific hooks
    // hooks with the same file name will override those from the global directory
    match host_dir.read_dir() {
        Ok(dirs) => {
            for entry in dirs {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        if entry.file_type().unwrap().is_file() {
                            hooks_files.insert(path.file_name().unwrap().to_os_string(), path);
                        }
                    }
                    Err(msg) => {
                        println!("{}", msg);
                    }
                }
            }
        }
        Err(msg) => {
            println!("{:?} {}", host_dir, msg);
        }
    }

    let mut keys = hooks_files.keys().collect::<Vec<&OsString>>();
    keys.sort();

    for file_name in keys {
        let path = hooks_files.get(file_name).unwrap();
        let s = path.as_os_str();


        if simulate {
            println!(":: Execute hook {:?}", path);
        } else {

            println!(":: Execute hook {:?}", file_name);
            let result = Command::new(s).spawn();
            match result {
                Ok(_) => (),
                Err(msg) => {
                    println!("::   --> failed: {}", msg);
                    return false;
                }
            }
        }
    }

    return true;



}
