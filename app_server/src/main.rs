use std::{env::args, net::SocketAddr, path::Path};

use http_parser::{Method, StatusCode};
use json_parser::save;
use smol_server::{Server};

mod handler;
use handler::*;

#[tokio::main]
async fn main() {
    println!("{:?}", args());
    println!("Hello, world!");

    let mut r_proxy = init().await;
    save_addr(r_proxy.get_stream().local_addr().unwrap());  //  Salva o endereço para o proxy conectar-se

    r_proxy.run().await;
}

fn save_addr(addr: SocketAddr) {

    let json_path: &Path = Path::new("../socket_addr.json");

    //  salva a porta do listener no json para o web_server conectar futuramente
    //  falha se der um erro horrendo, irrecuperável, não ser possível salvar no arquivo
    //  ou não ser possível converter pra json
    save(json_path, addr).unwrap();
}

async fn init() -> Server {
    use Method::*;

    let mut builder = Server::init("localhost:0").await;
    
    builder.add_fun(GET, "/pages/main_page/{*item}", get_item);
    builder.add_fun(POST, "/files/{*path}", post_item);
    builder.add_fun(GET, "/files", list_files);

    builder.build()
}



