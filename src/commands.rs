use std::path::PathBuf;

use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use crate::git::Git;

pub enum Command {
    GitPull,
    GitPush,
    GitCommitPush
}

impl Command {
    pub fn to_string(&self) -> String {
        match self {
            Command::GitPull => "git pull".to_string(),
            Command::GitPush => "git push".to_string(),
            Command::GitCommitPush => "git commit && git push".to_string()
        }
    }

    pub fn from_string(s: &str) -> Command {
        match s {
            "git pull\n" => Command::GitPull,
            "git push\n" => Command::GitPush,
            "git commit && git push\n" => Command::GitPush,
            _ => panic!("Invalid command"),
        }
    }

    pub fn to_string_vec() -> Vec<String> {
        vec![
            Command::GitPull.to_string(),
            Command::GitPush.to_string(),
            Command::GitCommitPush.to_string()
        ]
    }

    pub fn fzy_commands() {
        let selections = Command::to_string_vec();
        let input = FuzzySelect::with_theme(&ColorfulTheme::default())
            .with_prompt("What command do you want to run?")
            .default(0)
            .items(&selections[..])
            .interact()
            .unwrap();

        let input = selections[input].to_string() + "\n";
        let command = Command::from_string(&input);

        let path = std::env::current_dir().unwrap();
        match command {
            Command::GitPull => git_pull(path),
            Command::GitPush => git_push(path),
            Command::GitCommitPush => git_commit_push(path)
        }
    }
}

pub fn git_pull(path: PathBuf) {
    let git = Git::new(path);
    git.git_pull();
}

pub fn git_push(path: PathBuf) {
    let git = Git::new(path);
    git.git_commit_push();
}

pub fn git_commit_push(path: PathBuf) {
    let git = Git::new(path);
    git.git_commit_push();
}
