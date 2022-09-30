use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use tokio::task;

pub enum Commands {
    Start,
    Stop,
    Restart,
    Status,
    Exit,
}

impl Commands {
    pub fn to_string(&self) -> String {
        match self {
            Commands::Start => "Start",
            Commands::Stop => "Stop",
            Commands::Restart => "Restart",
            Commands::Status => "Status",
            Commands::Exit => "Exit",
        }
        .to_string()
    }

    pub fn from_string(s: &str) -> Commands {
        match s {
            "Start\n" => Commands::Start,
            "Stop\n" => Commands::Stop,
            "Restart\n" => Commands::Restart,
            "Status\n" => Commands::Status,
            "Exit\n" => Commands::Exit,
            _ => panic!("Invalid command"),
        }
    }

    pub fn to_string_vec() -> Vec<String> {
        vec![
            Commands::Start.to_string(),
            Commands::Stop.to_string(),
            Commands::Restart.to_string(),
            Commands::Status.to_string(),
            Commands::Exit.to_string(),
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
            Commands::Start => start(),
            Commands::Stop => stop(),
            Commands::Restart => restart(),
            Commands::Status => status(),
            Commands::Exit => exit(),
        }
    }
}

pub fn start() {
    println!("Starting...")
}

pub fn stop() {
    println!("Stopping...")
}

pub fn restart() {
    println!("Restarting...")
}

pub fn status() {
    println!("Status...")
}

pub fn exit() {
    println!("Exiting...")
}
