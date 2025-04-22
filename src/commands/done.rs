use crate::storage::json;

pub fn run(ids: Vec<u32>) {
    // Call the done_task function for each id, if one of them fails, print the error but continue
    for id in ids {
        match json::done_task(id) {
            Err(e) => {
                eprintln!("{}", e);
            },
            Ok(msg) => println!("{}", msg),
        }
    }
}