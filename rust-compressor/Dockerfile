# Build stage
FROM rust:latest AS builder

WORKDIR /app

# Copy manifest and source
COPY Cargo.toml .
COPY src ./src

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim AS runtime

WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/rust-compressor /app/rust-compressor

# Set the entrypoint to expose the CLI interface
ENTRYPOINT ["/app/rust-compressor"]
CMD ["--help"]
