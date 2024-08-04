use std::{env, path::Path, process};

pub fn cd(args: Vec<String>) {
    let path = match args.len() {
        0 => {
            // if args is empty, set cwd to home directory
            Path::new("/")
        }
        1 => {
            // otherwise try to set cwd to args[0]
            Path::new(args[0].as_str())
        }
        _ => {
            eprintln!("cd: too many args");
            return;
        }
    };

    // check if the given path exists before attempting to update the cwd
    if path.exists() {
        if env::set_current_dir(path).is_err() {
            eprintln!("Failed to change working directory to '{}'", path.display());
        }
    } else {
        eprintln!("cd: path '{}' does not exist", path.display())
    }
}

pub fn exit() {
    // cleanup

    // display exit message
    println!(":: Terminated crabshell ::");

    // exit crabshell
    process::exit(0);
}
