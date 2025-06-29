use clap::{Parser, Subcommand};
use taskclaw::task_manager::UUIDorIndex;

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
    /// List all tasks
    #[command(alias = "l")]
    List {
        /// Amount of tasks to show
        #[arg(short, long, default_value_t = 10)]
        amount: usize,
    },
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
        Some(Commands::Delete { id }) => {
            if let Some(task) = task_manager.remove_task(id) {
                println!("D {}", task);
            } else {
                eprintln!("Error: Task not found.");
                std::process::exit(1);
            }
        }
        Some(Commands::List { amount }) => {
            for task in task_manager.get_tasks_by_priority(amount) {
                println!("{}", task);
            }
        }
        None => {
            eprintln!("No command provided. Use --help for usage information.");
            std::process::exit(1);
        }
    }
}
