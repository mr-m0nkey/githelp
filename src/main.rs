use std::fs;
use std::env;
use std::path::Path;

enum OS {
    WINDOWS,
    LINUX
}

fn main() {

    let os = if cfg!(target_os = "windows") {
        OS::WINDOWS
    } else {
        OS::LINUX
    };

    let working_directory = env::current_dir().unwrap().into_os_string();


    for file in fs::read_dir(working_directory).unwrap() {
        let file = file.unwrap();
        if is_a_repo(&file) {
            println!("{}", file.path().display());
        }
    }

    
}

fn is_a_repo(file: &fs::DirEntry) -> bool {
    let path = file.path();
    let metadata = fs::metadata(&path).unwrap();
    if !metadata.is_dir() {
        return false;
    }

    let git_path = format!("{}/.git", path.display());

    return Path::new(&git_path).exists();
}


