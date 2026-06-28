FROM rust:1-bookworm AS builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

RUN cargo build --release --bin market-service --bin portfolio-service

FROM debian:bookworm-slim AS runtime
# ca-certificates for outbound HTTPS, service-to-service.
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/market-service /usr/local/bin/market-service
COPY --from=builder /app/target/release/portfolio-service /usr/local/bin/portfolio-service

CMD ["market-service"]
