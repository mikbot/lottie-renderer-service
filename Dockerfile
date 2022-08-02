FROM rust:1.62.1-slim-bullseye as builder
WORKDIR /usr/src/app
COPY . .

RUN ./scripts/build_rlottie.sh

RUN cargo build --release

FROM debian:bullseye-slim

COPY --from=builder /usr/src/app/target/release/lottie-renderer-service /
COPY --from=builder /usr/lib/librlottie.so* /usr/lib

ENTRYPOINT ["/lottie-renderer-service"]
