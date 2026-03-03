FROM rust:1-slim-bookworm AS builder

RUN apt-get update && apt-get install -y \
    valgrind \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY . .

CMD ["valgrind", "--leak-check=full", "cargo", "test", "--tests"]