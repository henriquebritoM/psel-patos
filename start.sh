#!/bin/bash

cd "$(dirname "$0")/app_server" || exit
cargo run --release

cd "$(dirname "$0")/web_server" || exit
cargo run --release