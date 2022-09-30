mod command_example;
use command_example::*;

fn main() {
    tokio_main(); //Async runtime
}

#[tokio::main]
async fn tokio_main() {
    Commands::fzy_commands();
}
