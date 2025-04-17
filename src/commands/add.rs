use crate::models::task::Task;
use crate::storage::json;
use chrono::{Utc, Duration};
use std::process;

pub fn run(description: String, due: Option<u32>, priority: Option<u32>) {
    // Parse the due date number into a correct date depending on local machine
    let due_date = match due {
        Some(days) => (Utc::now().naive_utc().date() + Duration::days(days as i64)).to_string(),
        None => "".to_string(),
    };

    // Priority is optional and defaults to 0
    let priority = if priority.unwrap_or(5) == 0 {
        eprintln!("Priority must be between 1 and 5");
        process::exit(1);
    } else {
        priority.unwrap_or(5)
    };

    // Create a new task
    match json::add_task(Task {
        id: 0, // ID will be assigned in the add_task function
        description,
        due_date: Some(due_date),
        priority: Some(priority),
        done: false,
    }) {
        Ok(task) => { task.print() },
        Err(err) => {
            eprintln!("{}", err); // todo: write a better error message in production
            process::exit(1);
        }
    }
}
