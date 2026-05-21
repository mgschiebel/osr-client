FROM rust:1.75 AS builder

WORKDIR /app
COPY rust/ .
RUN cargo build --bin mock-server --release

FROM debian:bookworm-slim

WORKDIR /app
COPY --from=builder /app/target/release/mock-server /app/mock-server
COPY --from=builder /app/rust/certs /app/certs

EXPOSE 8080

CMD ["/app/mock-server", "auth", "--port", "8080", "--cert", "/app/certs/cert.pem", "--key", "/app/certs/key.pem"]
