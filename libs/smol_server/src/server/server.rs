use std::{collections::HashMap, sync::{Arc, Mutex}};
use tokio::net::{TcpListener, ToSocketAddrs};

use matchit::Router;

use crate::server::{fn_handler::{BoxFallbackHandler, BoxHandler},server_builder::ServerBuilder, server_data::ServerData, stream_handler::ConectionHandler};
use crate::Client;

pub struct Server {
    listener: TcpListener,
    data: Arc<ServerData>,
    keep_alive: bool
}

//  Bloco de setters e getters
impl Server {
    pub async fn init<A: ToSocketAddrs>(addr: A) -> ServerBuilder {
        //  tcp listener para o servidor
        let listener = TcpListener::bind(addr).await.expect("Não foi possível conectar-se a porta");
        return ServerBuilder::create(listener, Router::new(), HashMap::new(), HashMap::new(), false);
    }

    // Torna apenas uma função publica para a crate, ao invés de tornar todos os campos de Server
    pub(crate) fn create(listener: TcpListener, router: Router<BoxHandler>, fallbacks: HashMap<u16, BoxFallbackHandler>, apis: Arc<HashMap<String, Arc<Mutex<Client>>>>, keep_alive: bool) -> Server {
        return Server { listener, data: Arc::new(ServerData::create(router, fallbacks, apis)), keep_alive };
    }

    pub fn get_stream(&mut self) -> &mut TcpListener {
        return &mut self.listener;
    }
    pub fn keep_alive(&mut self, keep_alive: bool) {
        self.keep_alive = keep_alive;
        // return req.headers.get_header_value("Connection") == Some("keep-alive".to_owned());
    }
}

impl Server {
    
    /// Liga o servidor. <br>
    /// Toma a ownership de self para evitar que os usuários da
    /// crate façam coisas bizarras
    pub async fn run(self) {

        //  Aceita diversos clients
        loop {
            println!("\n\n------------------------------\n\n");
            println!("waiting");
            let Ok((stream, _)) = self.listener.accept().await else {
                //  Ignora conexões falhas
                println!("Erro ao conectar-se a stream");
                continue;
            };
            
            let data_clone = self.data.clone();
            tokio::task::spawn(async move {ConectionHandler::handle(data_clone, stream)});

        }
    }
}