use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "claw",
    version = "0.1.0",
    author = "Robin Raymond <robin@robinraymond.de>",
    about = "A simple task manager CLI"
)]
struct Args {
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
}

fn main() {
    let args = Args::parse();
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
        None => {
            eprintln!("No command provided. Use --help for usage information.");
            std::process::exit(1);
        }
    }
}

