# Lottie Renderer Service

Small microservice to render [Lottie](https://en.wikipedia.org/wiki/Lottie_(file_format)) animation files via an http REST API.

## Run via docker
```
docker run -p 8080:8080 ghcr.io/mikbot/lottie-renderer-service
```

## Install via [crates.io](https://crates.io)

### rlottie

The [rlottie](https://github.com/Samsung/rlottie) library is required to build and run the service. 


#### Arch
Install via your favorite aur helper:
```shell
paru -S rlottie
```

#### Debian / Ubuntu
There is no up-to-date package available for debian based distros.
You can use [our shell script](./scripts/build_rlottie.sh) to install rlottie on your system.

```shell
git clone https://github.com/mikbot/lottie-renderer-service
cd lottie-renderer-service
./scripts/build_rlottie.sh
```

### Installation

```
cargo install lottie-renderer-service
lottie-renderer-service
```