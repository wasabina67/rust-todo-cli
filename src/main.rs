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
    }

    fn complete_task(&mut self, id: usize) -> Result<(), String> {
    }

    fn remove_task(&mut self, id: usize) -> Result<(), String> {
    }
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

fn main() {
    let cli = Cli::parse();
    let mut todo_list = load_todo_list();

    match cli.command {
        Commands::Add { description } => {
            println!("Add: {}", description);
        }
        Commands::List => {
            println!("List");
        }
        Commands::Complete { id } => {
            println!("Complete: id {}", id);
        }
        Commands::Remove { id } => {
            println!("Remove: id {}", id);
        }
    }
}
