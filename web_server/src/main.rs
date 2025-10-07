use std::{net::SocketAddr, path::Path};
use http_parser::{Method, Request, Response, StatusCode};
use json_parser::retrieve;
use smol_server::{Client, Params, Server, ServerBuilder};

use crate::erros_handlers::{bad_request, http_not_supported, not_allowed, not_found, server_error};

mod erros_handlers;

#[tokio::main]
async fn main() {
    println!("starting reverse proxy");

    let web_server = server_init().await;

    web_server.run().await;
}

async fn server_init() -> Server {
    let mut builder = Server::init("localhost:8080").await;
    let app = Client::init(get_addr()).await;
    init(&mut builder, app);
    return builder.build();
}

fn init(builder: &mut ServerBuilder, app: Option<Client>) {
    use Method::*;
    use StatusCode::*;

    builder.add_fun(GET, "/", redirect_to_index);
    builder.add_fun(GET, "/pages/{*item}", get_item);
    builder.add_fun(POST, "/files/{*item}", post_item);
    builder.add_fun(GET, "/files", list_files);

    builder.add_fallback_fun(NotFound, not_found);
    builder.add_fallback_fun(MethodNotAllowed, not_allowed);
    builder.add_fallback_fun(BadRequest, bad_request);
    builder.add_fallback_fun(HttpVersionNotSupported, http_not_supported);
    builder.add_fallback_fun(InternalServerError, server_error);

    if let Some(app) = app {
        builder.add_api("app", app);
    } else {
        eprintln!("Não foi possível conectar-se ao app server");
    }
}

async fn redirect_to_index(_req: Request, mut res: Response, _params: Params) -> Result<Response, StatusCode> {
    res.status = StatusCode::TemporaryRedirect;
    res.headers.add_header("Location", "/pages/main_page/index.html");
    Ok(res)
}

async fn get_item(req: Request, mut res: Response, params: Params) -> Result<Response, StatusCode> {
    res = params.api_fetch("app", req).await;
    Ok(res)
}

async fn post_item(req: Request, mut res: Response, params: Params) -> Result<Response, StatusCode> {
    res = params.api_fetch("app", req).await;
    Ok(res)
}

async fn list_files(req: Request, mut res: Response, params: Params) -> Result<Response, StatusCode> {
    res = params.api_fetch("app", req).await;
    Ok(res)
}

fn get_addr() -> SocketAddr {
    let json_path: &Path = Path::new("../socket_addr.json");

    let addr = retrieve(json_path).expect("Nenhum socket para conectar-se ao app server");
    return addr;
}

