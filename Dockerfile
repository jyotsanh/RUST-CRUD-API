# Stage 1: Build the Rust application
FROM rust:1.88-slim-bookworm AS builder

# Set the working directory
WORKDIR /usr/src/app

# Copy the source code
COPY . .

# Build the application in release mode
# Note: Docker will cache this step unless your source code changes
RUN cargo build --release

# Stage 2: Create a lightweight runtime image
FROM debian:bookworm-slim

WORKDIR /app

# Directory for the SQLite database (mounted as a volume in docker-compose)
RUN mkdir -p /app/data

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/actix-sqlite-api ./actix-server
# Expose the port your actix-web server is listening on
EXPOSE 8080

# Run the binary
CMD ["./actix-server"]
