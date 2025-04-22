use std::env;
use std::fmt::format;
use serde::{Serialize, Deserialize};
use colored::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub due_date: Option<String>,
    pub priority: Option<u32>,
    pub done: bool,
}

impl Task {
    pub fn new(id: u32, description: String, due_date: Option<String>, priority: Option<u32>) -> Task {
        Task {
            id,
            description,
            due_date,
            priority,
            done: false,
        }
    }

    pub fn print(&self) {
        // Load the environment variables from the .env file
        let text_color: String = env::var("TEXT_COLOR".to_string()).unwrap_or("white".to_string());
        let background: String = env::var("BACKGROUND_COLOR".to_string()).unwrap_or("none".to_string());

        // Format the task row
        let styled_row = format!(
            "{:<10} {:<50} {:<20} {:<10} {:<10}",
            self.id.to_string(),
            self.description.clone(),
            self.due_date.clone().unwrap(),
            self.priority.clone().unwrap(),
            self.done.to_string()
        );

        // Check the env vars and apply the appropriate color
        let styled = if background == "none" {
            styled_row.color(text_color)
        } else {
            styled_row.color(text_color).on_color(background)
        };

        // Print the row
        println!("{}", styled);
    }
}
