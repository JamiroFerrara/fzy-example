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
    
    pub fn git_add_all(&self) {
        Command::new("git").
            arg("add").
            arg("*").
            current_dir(&self.path).
            output().
            expect("Failed to add files to git");
    }

    pub fn git_push(&self) {
        Command::new("git").
            arg("push").
            current_dir(&self.path).
            output().
            expect("Failed to commit");
    }

    pub fn git_commit_push(&self) {
        self.git_add_all();

        Command::new("git").
            arg("commit").
            arg("-m").
            arg("upd").
            current_dir(&self.path).
            output().
            expect("Failed to commit");

        self.git_push();
    }
}
