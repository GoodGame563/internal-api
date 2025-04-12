FROM rust:1.86-slim-bullseye as builder

WORKDIR /app

RUN apt-get update && apt-get install -y protobuf-compiler && rm -rf /var/lib/apt/lists/*

COPY . .

RUN cargo build --release

FROM debian:bullseye-slim

WORKDIR /app

RUN apt-get update && \
    apt-get install -y libpq5 && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/internal-api /app/internal-api
COPY --from=builder /app/proto /app/proto
COPY --from=builder /app/.env-clear /app/.env

EXPOSE 50051

CMD ["./internal-api"]