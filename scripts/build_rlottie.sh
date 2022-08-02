#!/usr/bin/env bash

set -e

REPO="https://github.com/Samsung/rlottie"

if ! [ -x "$(command -v sudo)" ]; then
  SUDO=""
else
  SUDO="sudo"
fi

echo "Installing dependencies"
$SUDO apt update > /dev/null && $SUDO apt install -y pkg-config git cmake build-essential clang > /dev/null

if [ -d "rlottie" ]; then
    echo "rlottie is already cloned"
else
    echo "Cloning rlottie"
    git clone $REPO rlottie
fi

cd rlottie
git checkout v0.2

rm -rf build

echo "Building rlottie"
mkdir build && cd build
cmake ..
make
$SUDO make install
