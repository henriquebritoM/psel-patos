#!/bin/bash

echo "Iniciando o App Server"
cargo run --release --bin app_server &
APP_PID=$!  # salva o PID do processo do app

# espera um pouco para o app atualizar o arquivo socket_addr.json
sleep 3

echo "iniciando o Web Server"
cargo run --release --bin web_server 

echo "Desligando App e Web Servers"
kill $APP_PID
