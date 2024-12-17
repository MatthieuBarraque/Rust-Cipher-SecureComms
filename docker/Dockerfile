FROM rust:1.70-slim as builder

WORKDIR /usr/src/app

RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    echo "pub fn dummy() {}" > src/lib.rs

RUN cargo build --release

RUN rm src/main.rs src/lib.rs

COPY . .

RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update && \
    apt-get install -y openssl ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /usr/src/app/target/release/secure_chat /app/server
COPY --from=builder /usr/src/app/target/release/client /app/client
COPY --from=builder /usr/src/app/config.toml /app/config.toml

RUN mkdir -p /app/keys

ENV RUST_LOG=info
ENV CONFIG_PATH=/app/config.toml

EXPOSE 7878

COPY docker/entrypoint.sh /app/entrypoint.sh
RUN chmod +x /app/entrypoint.sh

ENTRYPOINT ["/app/entrypoint.sh"]