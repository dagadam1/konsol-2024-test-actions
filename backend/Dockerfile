# Build backend
FROM rust:1.90 AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY migrations ./migrations
COPY scripts ./scripts
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /app/target/release/backend .
COPY --from=builder /app/scripts .

RUN apt-get update && apt-get install -y libssl-dev ca-certificates sqlite3 libsqlite3-dev python3 && rm -rf /var/lib/apt/lists/*

EXPOSE 8080

CMD ["./backend"]