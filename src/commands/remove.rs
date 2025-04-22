use crate::storage::json;

pub fn run(mut ids: Vec<u32>, done: bool) {
    if done == true {
        // Fetch all id:s where done == true
        let mut done_ids: Vec<u32> = vec![];

        // Load all tasks from the json file
        let tasks = json::load_done_tasks().unwrap_or_else(|e| {
            eprintln!("{}", e);
            Vec::new() // Return an empty Vec if error occurs
        });

        // Collect all done task ids
        for task in tasks {
            done_ids.push(task.id);
        }

        // Overwrite the ids with the done ids
        ids = done_ids;
    }

    // Call the remove_task function for each id, if one of them fails, print the error but continue
    for id in ids {
        match json::remove_task(id) {
            Err(e) => {
                eprintln!("{}", e);
            },
            Ok(msg) => println!("{}", msg),
        }
    }
}