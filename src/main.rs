use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

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
}

fn main() {
    let cli = Cli::parse();
    let mut todo_list = TodoList::new();

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
