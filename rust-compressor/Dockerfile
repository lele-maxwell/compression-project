# Build stage
FROM rust:1.75 as builder

WORKDIR /app

# Copy manifest + lock file
COPY Cargo.toml Cargo.lock ./

# Copy full source
COPY . .

# Build the release binary
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim

WORKDIR /app

# Copy the compiled binary from builder
COPY --from=builder /app/target/release/rust-compressor /app/rust-compressor

# Set the binary as the entrypoint
ENTRYPOINT ["/app/rust-compressor"]
CMD ["--help"]
