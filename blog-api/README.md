# Blog API

A simple RESTful Blog API built with Rust, Axum, and SQLx.

## Features

- RESTful API for blog posts and users
- CRUD operations for posts and users
- PostgreSQL database with SQLx
- Async API with Tokio runtime
- JSON request and response handling
- Error handling with custom AppError type
- Configuration management with dotenv and config crates
- Structured logging with tracing

## Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- PostgreSQL
- Docker (optional, for containerized database)

## Setup

### Database

1. Set up PostgreSQL either locally or using Docker:

```bash
# Using Docker
docker run --name blog-postgres -e POSTGRES_PASSWORD=postgres -e POSTGRES_USER=postgres -e POSTGRES_DB=blog_db -p 5432:5432 -d postgres
```

### Environment

1. Create a `.env` file in the project root with the following content (adjust as needed):

```
DATABASE_URL=postgres://postgres:postgres@localhost:5432/blog_db
APP_SERVER__HOST=127.0.0.1
APP_SERVER__PORT=8080
RUST_LOG=info
```

## Running the application

1. Build and run the application:

```bash
cargo run
```

The server will start at http://127.0.0.1:8080 by default (or the port specified in your configuration).

## API Endpoints

### Posts

- `GET /api/posts` - List all posts (with pagination)
- `GET /api/posts/:id` - Get a specific post
- `POST /api/posts?author_id=<uuid>` - Create a new post
- `PUT /api/posts/:id` - Update a post
- `DELETE /api/posts/:id` - Delete a post
- `GET /api/posts/user/:user_id` - Get all posts by a specific user

### Users

- `GET /api/users` - List all users (with pagination)
- `GET /api/users/:id` - Get a specific user
- `POST /api/users` - Create a new user
- `PUT /api/users/:id` - Update a user
- `DELETE /api/users/:id` - Delete a user

### Health Check

- `GET /health` - Check if the API is running

## Example requests

### Create a user

```bash
curl -X POST http://localhost:8080/api/users \
  -H "Content-Type: application/json" \
  -d '{"username":"testuser","email":"test@example.com","password":"password123"}'
```

### Create a post

```bash
curl -X POST "http://localhost:8080/api/posts?author_id=<user-uuid>" \
  -H "Content-Type: application/json" \
  -d '{"title":"My First Post","content":"This is the content of my first blog post","published":true}'
```

## Development

### Running tests

```bash
cargo test
```

### Running with hot reload

For development, you can use `cargo-watch` to automatically rebuild and restart the server when files change:

```bash
cargo install cargo-watch
cargo watch -x run
```

## Project Structure

- `src/api/` - API routes and handlers
- `src/models/` - Data models and DTOs
- `src/db/` - Database connections and repositories
- `src/config/` - Application configuration
- `src/errors/` - Error handling
- `migrations/` - SQL migrations for database setup

## Future Improvements

- Add authentication with JWT
- Add more complex querying options
- Add tests for all endpoints
- Add OpenAPI/Swagger documentation
- Add rate limiting
- Add caching layer