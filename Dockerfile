FROM rust:1.62.1-alpine as builder
WORKDIR /usr/src/app
COPY . .
RUN apk add --no-cache musl-dev && cargo build --release

FROM scratch
COPY --from=builder /usr/src/app/target/release/lottie-renderer-service /
ENTRYPOINT ["/lottie-renderer-service"]
