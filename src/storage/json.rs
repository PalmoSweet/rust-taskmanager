use crate::models::task::Task;
use std::{fs, path::PathBuf};

pub fn get_path() -> PathBuf {
    // Creates a mutable path to the home directory
    let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));

    // Append the ".tasks.json" directory to the path (the filename)
    path.push(".tasks.json");

    // Returning the path to the file
    path
}

pub fn load() -> Result<Vec<Task>, String> {
    // Getting the path to the tasks file
    let path: PathBuf = get_path();

    // Check if the file exists, if not return an empty vector
    if !path.exists() {
        return Ok(vec![]);
    }

    // Tries to read the file and return an error if it fails
    let json_data: String = fs::read_to_string(&path)
        .map_err(|err| format!("Could not read {}: {}", path.display(), err))?;

    // Deserialize the JSON data into a vector of Task structs and return an error if it fails
    match serde_json::from_str(&json_data) {
        Ok(tasks) => Ok(tasks),
        Err(err) => Err(format!("Could not parse JSON file {}: {}", path.display(), err)),
    }
}

pub fn save(tasks: &[Task]) -> Result<String, String> {
    let path: PathBuf = get_path();

    // Serialize tasks and return potential error
    let json = serde_json::to_string_pretty(tasks)
        .map_err(|err| format!("Could not serialize tasks: {}", err))?;

    // Write the serialized JSON to the file and return a potential error (creates the file if it doesn't exist)
    fs::write(&path, json)
        .map_err(|err| format!("Could not write to {}: {}", path.display(), err))?;

    // Return success message if everything goes well
    Ok(format!("Tasks saved to {}", path.display()))
}


pub fn add_task(mut new_task: Task) -> Result<Task, String> {
    // Try to load tasks from the file, return an error if it fails
    let mut tasks: Vec<Task> = load()
        .map_err(|err: String| format!("Could not load tasks: {}", err))?;

    // Iterates over the tasks and finds the maximum ID, if no tasks exist, it defaults to 0 and adds 1
    new_task.id = tasks.iter().map(|t: &Task| t.id).max().unwrap_or(0) + 1;

    // Add the new task to the vector of tasks
    tasks.push(new_task.clone());

    // Save the tasks to the file, return an error if it fails
    save(&tasks)
        .map_err(|err: String| format!("Could not save tasks: {}", err))?;

    Ok(new_task)
}

// Removes a task by its ID
/*
pub fn remove_task(id: u32) {
    let mut tasks = load();
    let original_len = tasks.len();

    tasks.retain(|t| t.id != id);

    if tasks.len() == original_len {
        eprintln!("No task with ID {} found.", id);
    } else {
        save(&tasks);
        println!("Removed task with ID {}", id);
    }
}*/
