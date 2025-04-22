# Build stage
FROM rust:1.75 as builder

WORKDIR /app

# Copy manifest and lock file first (for caching)
COPY Cargo.toml Cargo.lock ./

# Create an empty src/main.rs just to make deps build not fail if src/ isn't copied yet
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo fetch

# Now copy actual source
COPY . .

# Build the release binary
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim

WORKDIR /app

# Copy the built binary
COPY --from=builder /app/target/release/rust-compressor /app/rust-compressor

# Set the entrypoint
ENTRYPOINT ["/app/rust-compressor"]
CMD ["--help"]
