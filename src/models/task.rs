use serde::{Serialize, Deserialize};

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
        println!("ID: {}, Description: {}, Due Date: {:?}, Priority: {:?}, Done: {}",
            self.id, self.description, self.due_date, self.priority, self.done);
    }
}
