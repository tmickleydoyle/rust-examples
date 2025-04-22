# Todo CLI

A simple command-line todo list application written in Rust. This application allows you to manage your tasks with basic functionality for adding, listing, and completing tasks. All tasks are stored in a local JSON file for persistence.

## Features

- Add new tasks with descriptions
- List all tasks (with optional filtering for completed or pending tasks)
- Mark tasks as complete
- Data persists between sessions in a local JSON file
- Each task has a unique ID, description, creation timestamp, and status

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.50 or newer)
- Cargo (comes with Rust)

### Building from Source

```bash
# Clone the repository (if applicable)
# git clone https://github.com/yourusername/todo-cli.git
# cd todo-cli

# Build the project
cargo build --release

# The binary will be available in target/release/todo-cli
```

## Usage

### Adding a Task

Add a new task to your todo list:

```bash
cargo run -- add "Buy groceries"
```

Or using the built binary:

```bash
./target/release/todo-cli add "Buy groceries"
```

You can add tasks with multi-word descriptions:

```bash
cargo run -- add "Call dentist for appointment"
```

### Listing Tasks

List all tasks in your todo list:

```bash
cargo run -- list
```

List only completed tasks:

```bash
cargo run -- list --completed
```

List only pending tasks:

```bash
cargo run -- list --pending
```

### Completing a Task

Mark a task as completed (you need the task ID from the list command):

```bash
cargo run -- complete <task-id>
```

For example:

```bash
cargo run -- complete 2690492c-0eaf-4941-8975-ec5f11617ab1
```

## Data Storage

Tasks are stored in a `tasks.json` file in the directory where you run the application. This file is created automatically when you add your first task.

## Project Structure

- `src/main.rs` - Main application logic
- `src/cli.rs` - Command-line interface definitions using clap
- `src/task.rs` - Task data structure and related methods
- `src/storage.rs` - File storage and persistence logic

## Dependencies

- `clap` - Command line argument parsing
- `serde` and `serde_json` - Serialization/deserialization of tasks
- `chrono` - Handling timestamps
- `uuid` - Generating unique IDs for tasks

## License

This project is open source and available under the [MIT License](LICENSE).