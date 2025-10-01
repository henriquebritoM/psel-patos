use std::{net::SocketAddr, path::Path};
use http_parser::{response, Method, Request, Response, StatusCode};
use json_parser::retrieve;
use smol_server::{Client, Server};

fn main() {
    println!("starting reverse proxy");

    let mut web_server = Server::init("localhost:8080");
    let app_server = Client::init(get_addr()).expect("Não foi possível conectar-se ao app server");

    init(&mut web_server);
    web_server.add_api("app", app_server);

    web_server.run();
}

fn init(server: &mut Server) {
    server.add_fun(Method::GET, "/", redirect_to_index);
    server.add_fun(Method::GET, "/pages/{*item}", get_item);
    server.add_fun(Method::POST, "/files/{*item}", post_item);
    // server.add_fun(Method::GET, "/pages/{ main_page/assets/js/script.js }", get_script);
}

fn redirect_to_index(_server: &mut Server, _req: &mut Request) -> Result<Response, StatusCode> {
    return Ok(Response::new().status(StatusCode::MovedPermanently).add_header("Location", "/pages/main_page/index.html").build());
}

fn get_item(server: &mut Server, req: &mut Request) -> Result<Response, StatusCode> {
    let response = server.api_send("app", req);
    println!("enviando para o app");
    Ok(response)
}

fn post_item(server: &mut Server, req: &mut Request) -> Result<Response, StatusCode> {
    let response = server.api_send("app", req);
    println!("enviando para o app");
    Ok(response)
}

fn get_addr() -> SocketAddr {
    let json_path: &Path = Path::new("../socket_addr.json");

    let addr = retrieve(json_path).expect("Nenhum socket para conectar-se ao app server");
    return addr;
}

