use clap::{Parser, Subcommand};

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

fn main() {
    let cli = Cli::parse();

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
