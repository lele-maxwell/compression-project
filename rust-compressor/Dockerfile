FROM rust:1.70 as builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
COPY --from=builder /app/target/release/rust-compressor /usr/local/bin/

ENTRYPOINT ["rust-compressor"] 