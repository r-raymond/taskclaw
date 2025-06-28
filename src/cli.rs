use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "claw")]
#[command(bin_name = "claw")]
#[command(about = "A simple CLI task tracking tool")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(author = "TaskClaw Contributors")]
#[command(long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new task
    Add {
        #[arg(help = "Task description")]
        description: String,
    },
    /// List all tasks
    List,
    /// Mark a task as complete
    Complete {
        #[arg(help = "Task ID to complete")]
        id: usize,
    },
    /// Remove a task
    Remove {
        #[arg(help = "Task ID to remove")]
        id: usize,
    },
    /// Generate shell completions
    Completions {
        #[arg(help = "Shell to generate completions for")]
        shell: String,
    },
    /// Launch interactive TUI mode
    Tui,
}
