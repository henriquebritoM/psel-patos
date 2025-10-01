use std::{env::args, net::SocketAddr, path::Path};

use http_parser::{Method, StatusCode};
use json_parser::save;
use smol_server::{Server};

mod handler;
use handler::*;
mod errors_handler;
use errors_handler::*;


fn main() {
    println!("{:?}", args());
    println!("Hello, world!");

    let mut r_proxy = Server::init("localhost:0");
    save_addr(r_proxy.get_stream().local_addr().unwrap());  //  Salva o endereço para o proxy conectar-se
    r_proxy.keep_alive(true);

    config(&mut r_proxy);

    r_proxy.run();
}

fn save_addr(addr: SocketAddr) {

    let json_path: &Path = Path::new("../socket_addr.json");

    //  salva a porta do listener no json para o web_server conectar futuramente
    //  falha se der um erro horrendo, irrecuperável, não ser possível salvar no arquivo
    //  ou não ser possível converter pra json
    save(json_path, addr).unwrap();
}

fn config(proxy: &mut Server) {
    use Method::*;
    use StatusCode::*;

    proxy.add_fun(GET, "/pages/main_page/{*item}", get_item);
    proxy.add_fun(POST, "/files/{*path}", post_item);


    proxy.add_fallback_fun(NotFound, not_found);
    proxy.add_fallback_fun(MethodNotAllowed, not_allowed);
    proxy.add_fallback_fun(BadRequest, bad_request);
    proxy.add_fallback_fun(HttpVersionNotSupported, http_not_supported);
    proxy.add_fallback_fun(InternalServerError, server_error);
}


