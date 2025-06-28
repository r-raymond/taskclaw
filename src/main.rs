use clap::{CommandFactory, Parser};
use clap_complete::{generate, shells::*};
use std::io;
use taskclaw::{
    cli::{Cli, Commands},
    config::load_config,
    task::{load_tasks, save_tasks},
    tui::run_tui,
};

fn main() {
    let cli = Cli::parse();
    let _config = load_config();
    let mut task_list = load_tasks();

    match cli.command {
        Commands::Add { description } => {
            task_list.add_task(description.clone());
            if let Err(e) = save_tasks(&task_list) {
                eprintln!("Warning: Could not save tasks: {}", e);
            }
            println!("Added task: {}", description);
        }
        Commands::List => {
            if task_list.tasks.is_empty() {
                println!("No tasks found.");
            } else {
                println!("Tasks:");
                for task in &task_list.tasks {
                    let status = if task.completed { "✓" } else { "○" };
                    println!("  {} [{}] {}", status, task.id, task.description);
                }
            }
        }
        Commands::Complete { id } => {
            if task_list.complete_task(id) {
                if let Err(e) = save_tasks(&task_list) {
                    eprintln!("Warning: Could not save tasks: {}", e);
                }
                println!("Completed task {}", id);
            } else {
                println!("Task {} not found", id);
            }
        }
        Commands::Remove { id } => {
            if task_list.remove_task(id) {
                if let Err(e) = save_tasks(&task_list) {
                    eprintln!("Warning: Could not save tasks: {}", e);
                }
                println!("Removed task {}", id);
            } else {
                println!("Task {} not found", id);
            }
        }
        Commands::Completions { shell } => {
            let mut cmd = Cli::command();
            match shell.to_lowercase().as_str() {
                "bash" => generate(Bash, &mut cmd, "claw", &mut io::stdout()),
                "zsh" => generate(Zsh, &mut cmd, "claw", &mut io::stdout()),
                "fish" => generate(Fish, &mut cmd, "claw", &mut io::stdout()),
                "powershell" => generate(PowerShell, &mut cmd, "claw", &mut io::stdout()),
                _ => {
                    eprintln!(
                        "Unsupported shell: {}. Supported shells: bash, zsh, fish, powershell",
                        shell
                    );
                    std::process::exit(1);
                }
            }
        }
        Commands::Tui => {
            match run_tui(task_list) {
                Ok(updated_task_list) => {
                    if let Err(e) = save_tasks(&updated_task_list) {
                        eprintln!("Warning: Could not save tasks: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("TUI error: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}
