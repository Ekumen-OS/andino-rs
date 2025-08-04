#!/bin/bash

export UID=$(id -u)
export GID=$(id -g)

docker build -t andino-rs-dev-container -f .devcontainer/rust-dev/Dockerfile \
    --build-arg UID=$UID \
    --build-arg GID=$GID \
    .
