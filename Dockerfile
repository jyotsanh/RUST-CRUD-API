# Stage 1: Build the Rust application
FROM rust:1.76-slim-bookworm AS builder

# Set the working directory
WORKDIR /usr/src/app

# Install necessary build dependencies (often needed for actix and mongodb drivers)
RUN apt-get update && apt-get install -y pkg-config libssl-dev

# Copy the source code
COPY . .

# Build the application in release mode
# Note: Docker will cache this step unless your source code changes
RUN cargo build --release

# Stage 2: Create a lightweight runtime image
FROM debian:bookworm-slim

# Install OpenSSL and CA certificates (required for network requests/DB connections)
RUN apt-get update && \
    apt-get install -y libssl3 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the compiled binary from the builder stage
# IMPORTANT: Replace "YOUR_PROJECT_NAME" with the actual name from your Cargo.toml
COPY --from=builder /usr/src/app/target/release/actix-mongo-api ./actix-server
# Expose the port your actix-web server is listening on
EXPOSE 8080

# Run the binary
CMD ["./actix-server"]