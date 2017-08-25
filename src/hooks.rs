use std::path::PathBuf;
use std::process::Command;
use std::collections::HashMap;
use std::ffi::{OsString};

pub fn run_hooks(dir: &PathBuf, host_dir: &PathBuf) {

    let mut hooks_files = HashMap::new();

    // collect the hooks from the main dir
    for entry in dir.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            let path = entry.path();
            if entry.file_type().unwrap().is_file() {
                hooks_files.insert(path.file_name().unwrap().to_os_string(), path);
            }
        }
    }

    // collect the host-specific hooks
    // hooks with the same file name will override those from the global directory
    for entry in host_dir.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            let path = entry.path();
            if entry.file_type().unwrap().is_file() {
                hooks_files.insert(path.file_name().unwrap().to_os_string(), path);
            }
        }
    }

    let mut keys = hooks_files.keys().collect::<Vec<&OsString>>();
    keys.sort();

    for file_name in keys {
        let path = hooks_files.get(file_name).unwrap();
        let s = path.as_os_str();
        println!("Running hook {:?}", file_name);

        let result = Command::new(s).spawn();
        match result {
            Ok(_) => (),
            Err(msg) => {
                println!("Hook failed: {}", msg);
            }
        }
    }



}
