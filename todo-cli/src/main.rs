mod cli;
mod storage;
mod task;

use clap::Parser;
use cli::{Cli, Commands};
use storage::Storage;
use task::Task;

fn main() {
    let cli = Cli::parse();
    let storage = Storage::new();

    match &cli.command {
        Commands::Add { description } => {
            let description = description.join(" ");
            let task = Task::new(description);
            
            match storage.add_task(task.clone()) {
                Ok(_) => {
                    println!("Added task: {} (ID: {})", task.description, task.id);
                }
                Err(e) => {
                    eprintln!("Failed to add task: {}", e);
                }
            }
        }
        Commands::List { completed, pending } => {
            match storage.load_tasks() {
                Ok(tasks) => {
                    if tasks.is_empty() {
                        println!("No tasks found.");
                        return;
                    }

                    let filtered_tasks: Vec<&Task> = tasks
                        .iter()
                        .filter(|task| {
                            if *completed {
                                task.is_completed()
                            } else if *pending {
                                !task.is_completed()
                            } else {
                                true
                            }
                        })
                        .collect();

                    if filtered_tasks.is_empty() {
                        println!("No matching tasks found.");
                        return;
                    }

                    println!("Tasks:");
                    for task in filtered_tasks {
                        let status = if task.is_completed() {
                            "[✓]"
                        } else {
                            "[ ]"
                        };
                        println!("{} {} - ID: {}", status, task.description, task.id);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to load tasks: {}", e);
                }
            }
        }
        Commands::Complete { id } => {
            match storage.complete_task(id) {
                Ok(true) => {
                    println!("Task marked as completed.");
                }
                Ok(false) => {
                    println!("Task with ID '{}' not found.", id);
                }
                Err(e) => {
                    eprintln!("Failed to complete task: {}", e);
                }
            }
        }
    }
}