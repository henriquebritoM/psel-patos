use std::{net::SocketAddr, path::Path};
use http_parser::{Method, Request, Response, StatusCode};
use json_parser::retrieve;
use smol_server::{Params, Server};

#[tokio::main]
async fn main() {
    println!("starting reverse proxy");

    let web_server = server_init().await;

    println!("running reverse proxy");
    web_server.run().await;
}

/// Inicia e configura o servidor de reverse proxy
async fn server_init() -> Server {
    use Method::*;

    let mut builder = Server::init("0.0.0.0:80").await;

    builder.add_fun(GET, "/", &redirect_to_index);
    builder.add_fun(GET, "/pages/{*item}", &get_item);
    builder.add_fun(GET, "/files/{*item}", &get_item);
    builder.add_fun(POST, "/files/{*item}", &post_item);
    builder.add_fun(GET, "/files", &list_files);

    builder.add_fun(OPTIONS, "/{*rest}",&resolve_options);

    builder.add_api("app", "backend:9000");

    return builder.build();
}

/// Envia uma response de redirect para a landing page
async fn redirect_to_index(_req: Request, mut res: Response, _params: Params) -> Result<Response, StatusCode> {
    res.status = StatusCode::TemporaryRedirect;
    res.headers.add_header("Location", "/pages/main_page/index.html");
    add_cors_headers(&mut res);
    Ok(res)
}

/// Encaminha a request para o App, para recuperar o arquivo do path
async fn get_item(req: Request, mut res: Response, params: Params) -> Result<Response, StatusCode> {
    println!("{:#?}", req);
    let Some(mut app) = params.get_api("app").await else {return Err(StatusCode::InternalServerError);};
    res = app.fetch(&req).await;
    add_cors_headers(&mut res);
    Ok(res)
}

/// Encaminha a request para o App, para a postagem de um novo arquivo
async fn post_item(req: Request, mut res: Response, params: Params) -> Result<Response, StatusCode> {
    println!("{:#?}", req);
    let Some(mut app) = params.get_api("app").await else {return Err(StatusCode::InternalServerError);};
    res = app.fetch(&req).await;
    add_cors_headers(&mut res);
    Ok(res)
}

/// Encaminha a request para o App, pedindo um json com o nome dos arquivos disponíveis
async fn list_files(req: Request, mut res: Response, params: Params) -> Result<Response, StatusCode> {
    println!("{:#?}", req);
    let Some(mut app) = params.get_api("app").await else {return Err(StatusCode::InternalServerError);};
    res = app.fetch(&req).await;
    add_cors_headers(&mut res);
    Ok(res)
}

async fn resolve_options(req: Request, mut res: Response, params: Params) -> Result<Response, StatusCode> {
    println!("{:#?}", req);
    add_cors_headers(&mut res);
    res.status(StatusCode::NoContent);
    Ok(res)
}

/// adiciona os headers necessários para o CORS policy
fn add_cors_headers(res: &mut Response) {
    res.add_header("Access-Control-Max-Age", 1728000);
    res.add_header("Access-Control-Allow-Origin", "*");
    res.add_header("Access-Control-Allow-Methods", "*");
    res.add_header("Access-Control-Allow-Headers", "DNT,User-Agent,X-Requested-With,If-Modified-Since,Cache-Control,Content-Type,Range");
}

/// Retorna o socket em que o App está rodando
fn get_addr() -> SocketAddr {

    let json_path: &Path = Path::new("./socket_addr.json");

    let addr = retrieve(json_path).expect("Nenhum socket para conectar-se ao app server");
    return addr;
}

