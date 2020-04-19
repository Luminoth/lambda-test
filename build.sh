#! /bin/sh

set -e

# https://github.com/softprops/lambda-rust
docker run --rm \
    -v ${PWD}:/code \
    -v ${HOME}/.cargo/registry:/root/.cargo/registry \
    -v ${HOME}/.cargo/git:/root/.cargo/git \
    softprops/lambda-rust

echo "Terraform to upload"
