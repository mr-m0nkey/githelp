use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

enum OS {
    WINDOWS,
    LINUX,
}

fn main() {
    let os = if cfg!(target_os = "windows") {
        OS::WINDOWS
    } else {
        OS::LINUX
    };

    let working_directory = env::current_dir().unwrap().into_os_string();

    for file in fs::read_dir("/Users/majagunna/Projects/rust").unwrap() {
        let file = file.unwrap();
        if is_a_repo(&file) {
            // println!("{}", file.path().display());
            run_command(
                &os,
                "git add .",
                &file.path().into_os_string().into_string().unwrap(),
            );
            run_command(
                &os,
                "git commit -m \"commit-message\"",
                &file.path().into_os_string().into_string().unwrap(),
            );
            run_command(
                &os,
                "git checkout -b new-branch-2",
                &file.path().into_os_string().into_string().unwrap(),
            );
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

fn run_command(os: &OS, command: &str, current_dir: &str) {
    println!("Runninf ({}) on ({})", &command, &current_dir);

    match os {
        OS::WINDOWS => {
            let mut formatted_command = String::from("/C ");
            formatted_command.push_str(&command);
            let command_tokens = formatted_command.split(" ").collect::<Vec<&str>>();
            let output = Command::new("cmd")
                .args(command_tokens)
                .current_dir(&current_dir)
                .output()
                .expect("failed to execute process");
            println!("status: {}", output.status);
            println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
            println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        }

        OS::LINUX => {
            let command_tokens = command.split(" ").collect::<Vec<&str>>();
            let output = Command::new(command_tokens.get(0).unwrap())
                .args(&command_tokens[1..])
                .current_dir(&current_dir)
                .output()
                .expect("failed to execute process");
            println!("status: {}", output.status);
            println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
            println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        }
    }
}
