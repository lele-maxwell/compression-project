# Build stage
FROM rust:1.75 as builder

WORKDIR /app

# Clean previous builds (if any)
RUN cargo clean

# Copy manifest + lock file
COPY Cargo.toml Cargo.lock ./

# Copy full source
COPY . .

# Build the release binary
RUN cargo build --release
