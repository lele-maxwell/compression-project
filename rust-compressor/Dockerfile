# Build stage
FROM 1.75 as builder

WORKDIR /app

# Copy source files
COPY Cargo.toml .
COPY . .

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim

WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/rust-compressor /app/rust-compressor

# Set the entrypoint to expose the CLI interface
ENTRYPOINT ["/app/rust-compressor"]
CMD ["--help"] 
