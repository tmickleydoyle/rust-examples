# CLI Todo App

## Prompt

Create a new Rust project for a command-line todo list application. The app should support adding new tasks, listing all tasks, and marking tasks as complete. Tasks should be stored in a local file using JSON or TOML for persistence. Each task should have a unique ID, a description, a creation timestamp, and a status indicating whether it's pending or completed. Use common Rust libraries for CLI argument parsing and data serialization. Organize the code in a modular way with separate files or modules for task definitions and storage logic. Include a README file in the project to explain how to use the todo commands.

## Pull Request

https://github.com/tmickleydoyle/rust-examples/pull/1

## Base sha

6eedb26ce1b6a794a22589f10f9fce734083a414

# Blog API App

## Prompt

Create a new Rust project for a Rust web backend project that implements a simple blog API with CRUD functionality for posts and users. The API should support endpoints for creating, retrieving, and updating blog posts. Use a popular Rust web framework like Actix Web or Axum, and handle requests and responses in JSON format. The project should be asynchronous, with proper error handling using Rust’s Result type. Data should be stored in a SQLite or Postgres database using an ORM like sqlx or diesel. Organize the code clearly and make use of Rust's type system and traits to keep the logic clean and maintainable. Include a README file in the project to explain how to use the todo commands. Create three example blog posts and a simple UI to view the posts.

## Pull Request

https://github.com/tmickleydoyle/rust-examples/pull/3

## Base sha

6eedb26ce1b6a794a22589f10f9fce734083a414

# Update Blog Border with Read or Unread

In the blog UI I want a blue border around each post, and I want the ability to mark each post as read and turn the border grey. This should happen on the client side.

## Pull Request

https://github.com/tmickleydoyle/rust-examples/pull/4

## Base sha

9ff0dcc14232106badca19d3179136836b39d2d5
