use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "A simple command-line todo app", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new task
    Add {
        /// Description of the task
        #[arg(required = true)]
        description: Vec<String>,
    },
    /// List all tasks
    List {
        /// Show only completed tasks
        #[arg(short, long)]
        completed: bool,

        /// Show only pending tasks
        #[arg(short, long)]
        pending: bool,
    },
    /// Mark a task as completed
    Complete {
        /// ID of the task to complete
        #[arg(required = true)]
        id: String,
    },
}