# Build stage: Use the Rust official image
FROM rust:latest AS builder

# Set the working directory inside the container
WORKDIR /usr/src/rust

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to cache dependencies
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs

# Fetch and cache dependencies
RUN cargo fetch

# Copy the actual source code
COPY . .

# Build the application in release mode
RUN cargo build --release

# Runtime stage: Use a minimal base image
FROM debian:bullseye-slim

# Install any runtime dependencies
RUN apt-get update && apt-get install -y libssl-dev && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /usr/local/bin

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/rust/target/release/rust .

# Define the default command
CMD ["./rust"]
