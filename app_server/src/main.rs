use std::{env::args, net::SocketAddr, path::Path};

use http_parser::Method;
use json_parser::save;
use smol_server::Server;

mod handler;
use handler::*;

#[tokio::main]
async fn main() {
    println!("{:?}", args());
    println!("Starting App Server");

    let app_server = init().await;
    println!("App server started!");
    // save_addr(app_server.get_stream().local_addr().unwrap());

    app_server.run().await;
}

///  Salva o endereço em um .json para o proxy conectar-se
fn save_addr(addr: SocketAddr) {
    let json_path: &Path = Path::new("./socket_addr.json");

    //  salva a porta do listener no json para o web_server conectar futuramente
    //  falha se der um erro horrendo, irrecuperável, não ser possível salvar no arquivo
    //  ou não ser possível converter pra json
    save(json_path, addr).unwrap();
}

/// Inicia e configura o Servidor
async fn init() -> Server {
    use Method::*;

    //  Inicia o servidor na porta 80
    let mut builder = Server::init("0.0.0.0:9000").await;

    builder.add_fun(GET, "/pages/main_page/{*item}", &get_item);
    builder.add_fun(POST, "/files/{*path}", &post_item);
    builder.add_fun(GET, "/files/{*path}", &get_item);
    builder.add_fun(GET, "/files", &list_files);

    builder.build()
}
