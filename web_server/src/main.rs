use std::{net::SocketAddr, path::Path};
use http_parser::{Method, Request, Response, StatusCode};
use json_parser::retrieve;
use smol_server::{Client, Params, Server};

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
    server.add_fun(Method::GET, "/files", list_files);
    // server.add_fun(Method::GET, "/pages/{ main_page/assets/js/script.js }", get_script);
}

fn redirect_to_index(_req: Request, res: &mut Response, _params: Params) -> Result<(), StatusCode> {
    res.status = StatusCode::TemporaryRedirect;
    res.headers.add_header("Location", "/pages/main_page/index.html");
    Ok(())
}

fn get_item(req: Request, res: &mut Response, params: Params) -> Result<(), StatusCode> {
    *res = params.api_fetch("app", req);
    Ok(())
}

fn post_item(req: Request, res: &mut Response, params: Params) -> Result<(), StatusCode> {
    println!("Recebido: {:?}", req);
    *res = params.api_fetch("app", req);
    println!("enviando para o app");
    Ok(())
}

fn list_files(req: Request, res: &mut Response, params: Params) -> Result<(), StatusCode> {
    println!("Recebido: {:?}", req);
    *res = params.api_fetch("app", req);
    println!("enviando para o app");
    Ok(())
}

fn get_addr() -> SocketAddr {
    let json_path: &Path = Path::new("../socket_addr.json");

    let addr = retrieve(json_path).expect("Nenhum socket para conectar-se ao app server");
    return addr;
}

