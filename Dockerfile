FROM rust:1.80-alpine AS builder

WORKDIR /app
COPY Cargo.toml ./
COPY src/ ./src/
COPY tests/ ./tests/

RUN cargo build --release

FROM alpine:3.20

WORKDIR /root/
COPY --from=builder /app/target/release/rust-zk-snark-verifier-engine /usr/local/bin/zk-verifier

ENTRYPOINT ["zk-verifier"]
