use std::env;
use serde::{Serialize, Deserialize};
use colored::*;
use textwrap::wrap;

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
        let text_color: String = env::var("TEXT_COLOR").unwrap_or_else(|_| "white".to_string());
        let background: String = env::var("BACKGROUND_COLOR").unwrap_or_else(|_| "none".to_string());

        // Wrap the description text to fit into 50 characters
        let wrapped_description = wrap(&self.description, 50);

        // Format the first line with all columns
        let first_line = format!(
            "{:<10} {:<50} {:<20} {:<10} {:<10}",
            self.id.to_string(),
            wrapped_description[0],
            self.due_date.clone().unwrap_or_default(),
            self.priority.clone().unwrap_or_default(),
            self.done.to_string()
        );

        // Format remaining lines with just the description, indented to align
        let other_lines: Vec<String> = wrapped_description
            .iter()
            .skip(1)
            .map(|line| format!("{:<10} {:<50}", "", line))  // empty ID column + wrapped text
            .collect();

        // Apply colors
        let styled = if background == "none" {
            std::iter::once(first_line.color(text_color.to_string()))
                .chain(other_lines.iter().map(|l| l.color(text_color.to_string())))
                .collect::<Vec<_>>()
        } else {
            std::iter::once(first_line.color(text_color.to_string()).on_color(background.to_string()))
                .chain(other_lines.iter().map(|l| l.color(text_color.to_string()).on_color(background.to_string())))
                .collect::<Vec<_>>()
        };

        // Print all lines
        for line in styled {
            println!("{}", line);
        }
    }
}
