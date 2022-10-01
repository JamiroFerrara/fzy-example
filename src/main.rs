pub mod commands;
pub mod git;

use commands::*;

fn main() {
    tokio_main(); //Async runtime
}

#[tokio::main]
async fn tokio_main() {
    Commands::fzy_commands();
}
