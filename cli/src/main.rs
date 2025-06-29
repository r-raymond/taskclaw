use clap::{Parser, Subcommand};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Parser, Debug)]
#[command(
    name = "claw",
    version = "0.1.0",
    author = "Robin Raymond <robin@robinraymond.de>",
    about = "A simple task manager CLI"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Clone)]
enum UUIDorIndex {
    UUID(String),
    Index(usize),
}

impl FromStr for UUIDorIndex {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(idx) = s.parse::<usize>() {
            Ok(UUIDorIndex::Index(idx))
        } else {
            // Optionally, validate UUID format here
            Ok(UUIDorIndex::UUID(s.to_string()))
        }
    }
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Add a new task
    #[command(alias = "a")]
    Add {
        /// The title of the task to add
        title: Vec<String>,
    },
    /// Delete a task
    #[command(alias = "d")]
    Delete { id: UUIDorIndex },
}

fn main() {
    let args = Cli::parse();
    let mut task_manager = taskclaw::task_manager::TaskManager::new();

    match args.command {
        Some(Commands::Add { title }) => {
            let title = title.join(" ");
            if title.is_empty() {
                eprintln!("Error: Task title cannot be empty.");
                std::process::exit(1);
            }
            let task = task_manager.create_task(title);
            println!("{}", task);
        }
        Some(Commands::Delete { id }) => match id {
            UUIDorIndex::UUID(uuid_str) => {
                if let Ok(uuid) = Uuid::parse_str(&uuid_str) {
                    task_manager.remove_task_by_uuid(&uuid);
                    println!("Task with UUID {} deleted.", uuid);
                } else {
                    eprintln!("Error: Invalid UUID format.");
                    std::process::exit(1);
                }
            }
            UUIDorIndex::Index(index) => {
                task_manager.remove_task_by_index(index);
                println!("Task at index {} deleted.", index);
            }
        },
        None => {
            eprintln!("No command provided. Use --help for usage information.");
            std::process::exit(1);
        }
    }
}

