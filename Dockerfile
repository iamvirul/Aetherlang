FROM rust:latest as builder

WORKDIR /app

COPY Cargo.toml .
COPY Cargo.lock .
COPY src ./src

RUN echo "Building at $(date)" > build_time.txt
RUN cargo version
RUN rustup update stable
RUN cargo version
RUN cargo build --release

RUN echo "Building at $(date)" > build_time.txt
RUN cargo version
RUN rustup update stable
RUN cargo version
RUN cargo build --release

FROM debian:bullseye-slim

WORKDIR /app

COPY --from=builder /app/target/release/aeth .

CMD ["./aeth"]
