#!/bin/bash

rm -rf target/
docker run --rm --user "$(id -u)":"$(id -g)" \
       -v "$(pwd)":/usr/src/myapp \
       -w /usr/src/myapp \
       rust:1.51.0-alpine cargo build --release
