use std::{env, process};
use chrono::{Utc, NaiveDate, Duration};
use colored::Colorize;
use crate::storage;
use crate::models::task::Task;

fn print_tasks(tasks: Vec<Task>) {
    // Load the environment variables from the .env file
    let text_color: String = env::var("HEADER_COLOR".to_string()).unwrap_or("white".to_string());
    let background: String = env::var("HEADER_BACKGROUND_COLOR".to_string()).unwrap_or("none".to_string());

    // Print the tasks in a table format
    let header = format!(
        "{:<10} {:<50} {:<20} {:<10} {:<10}",
        "ID", "DESCRIPTION", "DUE DATE", "PRIORITY", "DONE",
    );

    // Check the env vars and apply the appropriate color
    let styled_header = if background == "none" {
        header.color(text_color)
    } else {
        header.color(text_color).on_color(background)
    };

    // Print the header
    println!("{}", styled_header);

    // Print each task
    for task in tasks {
        task.print();
    }
}
pub fn run(today: bool, tomorrow: bool, undone: bool, done: bool) {
    // Load all the tasks from the json file
    let mut tasks: Vec<Task> = match storage::json::load() {
        Ok(tasks) => tasks,
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    };

    // Sort the tasks by priority
    tasks.sort_by_key(|task| task.priority);

    // Filter the tasks depending on the flags
    if today || tomorrow || undone || done {
        let filtered_tasks: Vec<Task> = tasks.into_iter()
            .filter(|task: &Task| {
                #[allow(unused_assignments)] // Warning about keep never read but its necessary for the filter
                let mut keep: bool = false;

                keep = (done && task.done) || (undone && !task.done);

                if today {
                    if let Some(ref date_str) = task.due_date {
                        if let Ok(due_date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
                            if due_date == Utc::now().date_naive() {
                                keep = true;
                            }
                        }
                    }
                }

                if tomorrow {
                    if let Some(ref date_str) = task.due_date {
                        if let Ok(due_date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
                            if due_date == Utc::now().date_naive() + Duration::days(1) {
                                keep = true;
                            }
                        }
                    }
                }

                keep
            })
            .collect();

        // Print the filtered tasks
        print_tasks(filtered_tasks);
    }
    else {
        // Print all tasks
        print_tasks(tasks);
    }
}