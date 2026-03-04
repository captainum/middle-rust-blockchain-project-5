FROM rust:1-slim-bookworm AS builder

RUN apt-get update && apt-get install -y \
    valgrind \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY src ./src
COPY tests ./tests
COPY benches ./benches

COPY entrypoint.sh ./entrypoint.sh

RUN chmod +x ./entrypoint.sh
CMD ["sh", "./entrypoint.sh"]
