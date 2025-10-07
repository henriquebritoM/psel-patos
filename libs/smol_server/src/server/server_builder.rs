use std::collections::HashMap;
use std::net::{SocketAddr, ToSocketAddrs};

use http_parser::{Method, StatusCode};
use matchit::{Router};
use tokio::net::TcpListener;

use crate::server::fn_handler::{FallbackHandler, FnHandler};
use crate::server::server_data::{ServerData, ToStatic};
use crate::{server::fn_handler::{BoxFallbackHandler, BoxHandler}, Params, Server};

/// struct para construção de um Server
/// Esse modele de construtor impede que os campos 
/// do Server sejam modificados após sua criação
pub struct ServerBuilder {
    listener: TcpListener,
    router: Router<(BoxHandler, Params)>,                  //  Uma URL router
    fallbacks: HashMap<u16, BoxFallbackHandler>,  
    apis: HashMap<String, SocketAddr>,
    temp_fns: Vec<(String, BoxHandler)>               //  só é possível colocar no router durante o build
}

impl ServerBuilder {

    // Torna apenas uma função publica para a crate, ao invés de tornar todos os campos de ServerBuilder
    pub(crate) fn create(listener: TcpListener) -> ServerBuilder {
        return ServerBuilder { listener, router: Router::new(), fallbacks: HashMap::new(), apis: HashMap::new(), temp_fns: Vec::new() };
    }

    /// Adiciona uma função padrão que é executada quando <br>
    /// uma request chegar o path e método passados <br>
    /// Parametros passados no path podem ser acessados pelas funções
    /// através da struct Params
    /// 
    /// # PANICS
    /// se o path não for válido
    pub fn add_fun<F>(&mut self, method: Method, path: &str, f: &'static F) 
    where
        F: FnHandler
    {
        let key = ServerData::get_router_path(method, path);

        self.temp_fns.push((key, f));  
    }

    /// Adiciona uma função de fallbacks. <br>
    /// Essas funções são executadas automaticamente 
    /// quando há um problema na request ou <br>
    /// quando uma função padrão retorna um StatusCode <br>
    /// Adequadas para **definir Responses padrões para erros**
    pub fn add_fallback_fun<F, Fut>(&mut self, status_code: StatusCode, f: &'static F) 
    where
        F: FallbackHandler 
    {
        let key = ServerData::get_fallback_hash_key(&status_code);

        self.fallbacks.insert(key, f);
    }

    /// Adiciona uma api ao seu servidor. <br>
    /// apis podem ser acessadas dentro das funções padrões 
    /// através da struct Params
    /// 
    /// # PANICS
    /// se o addr passado for inválido
    pub fn add_api<T: ToSocketAddrs>(&mut self, name: &str, addr: T) {
        let mut iter = addr.to_socket_addrs().expect("Socket inválido");
        let socket = iter.next().expect("Socket inválido");

        self.apis.insert(name.to_string(), socket);
    }

    /// Conclui a criação e retorna uma instância de Server
    pub fn build(mut self) -> Server {
        let apis_leak = self.apis.to_static();

        let p = Params{ apis: apis_leak, arguments: HashMap::new() };    

        for (key, f) in self.temp_fns {
            self.router.insert(key, (f, p.clone())).unwrap(); //    Mandar a mensagem de erro do matchit
        };
        return Server::create(self.listener, self.router, self.fallbacks);
    }
}

