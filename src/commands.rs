use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use crate::git::Git;

pub enum Commands {
    GitPull,
    GitPush
}

impl Commands {
    pub fn to_string(&self) -> String {
        match self {
            Commands::GitPull => "git pull".to_string(),
            Commands::GitPush => "git push".to_string()
        }
    }

    pub fn from_string(s: &str) -> Commands {
        match s {
            "git pull\n" => Commands::GitPull,
            "git push\n" => Commands::GitPush,
            _ => panic!("Invalid command"),
        }
    }

    pub fn to_string_vec() -> Vec<String> {
        vec![
            Commands::GitPull.to_string(),
            Commands::GitPush.to_string()
        ]
    }

    pub fn fzy_commands() {
        let selections = Commands::to_string_vec();
        let input = FuzzySelect::with_theme(&ColorfulTheme::default())
            .with_prompt("What command do you want to run?")
            .default(0)
            .items(&selections[..])
            .interact()
            .unwrap();

        let input = selections[input].to_string() + "\n";
        let command = Commands::from_string(&input);

        match command {
            Commands::GitPull => git_pull(),
            Commands::GitPush => git_push(),
        }
    }
}

pub fn git_pull() {
    let git = Git::new(std::env::current_dir().unwrap());
    git.git_pull();
}

pub fn git_push() {
    let git = Git::new(std::env::current_dir().unwrap());
    git.git_commit_push();
}
