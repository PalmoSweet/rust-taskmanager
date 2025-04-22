pub mod commands;
pub mod storage;
pub mod models;
pub mod cli;

use dirs::home_dir;

pub fn run() {
    // Loading the environment variables from the .env file
    //dotenv::dotenv().ok();
    if let Some(home) = home_dir() {
        let env_file = home.join(".task.env"); // Expands to the full path
        dotenv::from_filename(env_file.to_str().unwrap()).ok();
    }

    // Running the cli
    cli::run();
}