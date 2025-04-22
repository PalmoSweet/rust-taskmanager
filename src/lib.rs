pub mod commands;
pub mod storage;
pub mod models;
pub mod cli;

pub fn run() {
    // Loading the environment variables from the .env file
    dotenv::dotenv().ok();

    // Running the cli
    cli::run();
}