FROM rust:1.62.1-slim-bullseye as builder
WORKDIR /usr/src/app
COPY . .

# Build rust code
RUN apt update && apt install -y pkg-config git cmake make build-essential clang

RUN git clone https://github.com/Samsung/rlottie \
  && cd rlottie && git checkout 875626965959d8e269ca22175c8e1ad190696c43 \
  && mkdir build && cd build \
  && cmake .. \
  && make -j 2 \
  && make install

RUN cargo build --release

FROM debian:stable-slim

COPY --from=builder /usr/src/app/target/release/lottie-renderer-service /
COPY --from=builder /usr/lib/librlottie.so /usr/lib
COPY --from=builder /usr/lib/librlottie.so.0 /usr/lib
COPY --from=builder /usr/lib/librlottie.so.0.2 /usr/lib
ENTRYPOINT ["/lottie-renderer-service"]
