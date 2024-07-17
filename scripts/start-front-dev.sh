#!/usr/bin/env bash

make js-dev &
make css-dev &
wait
cargo run -p front
