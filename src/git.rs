use std::path::PathBuf;
use std::process::Command;

pub struct Git {
    path: PathBuf
}

impl Git {
    pub fn new(path: PathBuf) -> Git {
        Git {
            path
        }
    }

    pub fn git_pull(&self) {
        let mut git = Command::new("git");
        git.arg("pull");
        git.current_dir(&self.path);
        match git.output() {
            Ok(output) => {
                println!("{}", String::from_utf8_lossy(&output.stdout));
            },
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    pub fn git_commit_push(&self) {
        let mut git = Command::new("git");
        git.arg("add *");
        git.current_dir(&self.path);
        match git.output() {
            Ok(output) => {
                println!("{}", String::from_utf8_lossy(&output.stdout));
            },
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
