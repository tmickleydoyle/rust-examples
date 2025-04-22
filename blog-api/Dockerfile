FROM rust:1.75-slim as builder

WORKDIR /app

# Create a dummy project to cache dependencies
RUN cargo new --bin dummy
WORKDIR /app/dummy
COPY Cargo.toml .
RUN cargo build --release
RUN rm -rf src target/release/deps/blog_api*

# Build the actual project
WORKDIR /app
COPY . .
RUN cargo build --release

# Final image
FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates libssl-dev libpq-dev \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/blog-api /usr/local/bin/
COPY --from=builder /app/migrations /migrations
COPY --from=builder /app/config /config

EXPOSE 8080

CMD ["blog-api"]