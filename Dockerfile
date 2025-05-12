FROM rust:latest as builder

WORKDIR /app

COPY Cargo.toml .
COPY Cargo.lock .
COPY src ./src

RUN cargo build --release

FROM ubuntu:latest

WORKDIR /app

COPY --from=builder /app/target/release/aeth .

CMD ["./aeth"]
