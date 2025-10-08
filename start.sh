#!/bin/bash

cargo build --release

./target/release/app_server.d
./target/release/web_server.d