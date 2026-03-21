FROM rust:1.88-alpine AS builder
ENV RUSTFLAGS="-C target-feature=-crt-static"

RUN apk add --no-cache musl-dev

WORKDIR /app

COPY ./ /app

RUN cargo build --release
RUN strip target/release/rs-shortener

FROM alpine:3.19
RUN apk add --no-cache libgcc
COPY --from=builder /app/target/release/rs-shortener .
ENTRYPOINT ["/rs-shortener"]
