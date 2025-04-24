# Builder Stage
FROM rust:1.86-slim AS builder

WORKDIR /usr/src/app 
COPY . .

RUN cargo build --release --bin http_server

# Runtime Stage
FROM debian:bookworm-slim

COPY --from=builder /usr/src/app/target/release/http_server /usr/local/bin/http_server

EXPOSE 8080

CMD ["http_server"]