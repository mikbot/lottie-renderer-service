# Lottie Renderer Service

Small microservice to render [Lottie](https://en.wikipedia.org/wiki/Lottie_(file_format)) animation files via an http REST API.

## Run via docker
```
docker run -p 8080:8080 ghcr.io/mikbot/lottie-renderer-service
```

## Install via [crates.io](https://crates.io)
The `rlottie` lib is required, you can obtain it from [AUR](https://aur.archlinux.org/packages/rlottie), [dpkg](https://packages.debian.org/bullseye/librlottie0-1) or [build from source](https://github.com/Samsung/rlottie#building-lottie)
```
cargo install lottie-renderer-service
lottie-renderer-service
```
