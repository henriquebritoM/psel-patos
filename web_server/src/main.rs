use std::{net::SocketAddr, path::Path};
use http_parser::{Method, Request, Response, StatusCode};
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
    server.add_fun(Method::GET, "/", get_index);
    server.add_fun(Method::GET, "/assets/js/script.js", get_script);
    server.add_fun(Method::GET, "/assets/css/style.css", get_css);
    server.add_fun(Method::GET, "/assets/image.jpg", get_image);
}

fn get_index(server: &mut Server, req: &mut Request) -> Result<Response, StatusCode> {
    let response = server.api_send("app", req);
    Ok(response)
}

fn get_css(server: &mut Server, req: &mut Request) -> Result<Response, StatusCode> {
    let response = server.api_send("app", req);
    Ok(response)
}

fn get_image(server: &mut Server, req: &mut Request) -> Result<Response, StatusCode> {
    let response = server.api_send("app", req);
    Ok(response)
}

fn get_script(server: &mut Server, req: &mut Request) -> Result<Response, StatusCode> {
    let response = server.api_send("app", req);
    Ok(response)
}

fn get_addr() -> SocketAddr {
    let json_path: &Path = Path::new("../socket_addr.json");

    let addr = retrieve(json_path).expect("Nenhum socket para conectar-se ao app server");
    return addr;
}

