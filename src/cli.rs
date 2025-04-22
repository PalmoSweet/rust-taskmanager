use clap::{Parser, Subcommand};
use crate::commands;

#[derive(Parser)]
#[command(name = "task")]
#[command(about = "A command-line task manager written in rust")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(alias = "a", about = "Add a new task to the list.")]
    Add {
        // The description of the task
        #[arg(help = "The description of the task.")]
        description: String,

        // The due date of the task, optional and default to 0
        #[arg(short, long, num_args = 0..=1, default_missing_value = "0", help = "The amount of days forward for the due date (default is 0).")]
        due: Option<u32>,

        // Task priority (lower = more important)
        #[arg(short, long, help = "The priority of the task (lower numbers are higher priority).")]
        priority: Option<u32>,
    },
    #[command(alias = "ls", about = "List all of your tasks.")]
    List {
        #[arg(short, long, help = "List today's tasks")]
        today: bool,

        #[arg(short = 'm', long, help = "List tomorrow's tasks")]
        tomorrow: bool,

        #[arg(short, long, help = "List undone tasks")]
        undone: bool,

        #[arg(short, long, help = "List done tasks")]
        done: bool,
    },
    #[command(alias = "rm", about = "Remove a task from the list.")]
    Remove {
        #[arg(help = "The ID of the task to remove.")]
        ids: Vec<u32>,

        #[arg(short, long, help = "Remove all tasks that are marked as done.")]
        done: bool,
    },
    #[command(alias = "d", about = "Mark a task as done.")]
    Done {
        #[arg(help = "The ID of the task to mark as done.")]
        ids: Vec<u32>,
    },
    #[command(alias = "ud", about = "Mark a task as usdone.")]
    Undone {
        #[arg(help = "The ID of the task to mark as undone.")]
        ids: Vec<u32>,
    }
}

pub fn run() {
    let cli: Cli = Cli::parse();

    match cli.command {
        Commands::Add { description, due, priority } => commands::add::run(description, due, priority),
        Commands::List {today, tomorrow, undone, done} => commands::list::run(today, tomorrow, undone, done),
        Commands::Remove {ids, done} => commands::remove::run(ids, done),
        Commands::Done {ids} => commands::done::run(ids),
        Commands::Undone {ids} => commands::done::run(ids),
    }
}