use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "todo-cli")]
#[command(about = "todo-cli", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add
    Add { description: String },
    /// List
    List,
    /// Complete
    Complete { id: usize },
    /// Remove
    Remove { id: usize },
}

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct TodoList {
    tasks: Vec<Task>,
    next_id: usize,
}

impl TodoList {
    fn new() -> Self {
        TodoList {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, description: String) {
    }

    fn list(&mut self) {
        if self.tasks.is_empty() {
            println!("No tasks");
            return;
        }

        println!();
        for task in &self.tasks {
            let status = if task.completed { "✓" } else { " " };
            println!("[{}] {} - {}", status, task.id, task.description);
        }
        println!();
    }

    // fn complete_task(&mut self, id: usize) -> Result<(), String> {
    // }

    // fn remove_task(&mut self, id: usize) -> Result<(), String> {
    // }
}

fn get_data_file_path() -> PathBuf {
    let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push(".todo-cli.json");
    path
}

fn load_todo_list() -> TodoList {
    let path = get_data_file_path();
    if path.exists() {
        let data = fs::read_to_string(&path).expect("Failed to read file");
        serde_json::from_str(&data).expect("Failed to parse JSON")
    } else {
        TodoList::new()
    }
}

fn save_todo_list(todo_list: &TodoList) {
    let path = get_data_file_path();
    let data = serde_json::to_string_pretty(todo_list).expect("Failed to convert to JSON");
    fs::write(&path, data).expect("Failed to write file");
}

fn main() {
    let cli = Cli::parse();
    let mut todo_list = load_todo_list();

    match cli.command {
        Commands::Add { description } => {
            println!("Add: {}", description);
        }
        Commands::List => {
            todo_list.list();
        }
        Commands::Complete { id } => {
            println!("Complete: id {}", id);
        }
        Commands::Remove { id } => {
            println!("Remove: id {}", id);
        }
    }
}
