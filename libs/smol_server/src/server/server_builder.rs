use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use futures::future::BoxFuture;
use http_parser::{Request, Response};
use http_parser::{Method, StatusCode};
use matchit::{Router};
use tokio::net::TcpListener;

use crate::server::server_data::ServerData;
use crate::{server::fn_handler::{BoxFallbackHandler, BoxHandler, Result}, Client, Params, Server};

/// struct para construção de um Server
/// Esse modele de construtor impede que os campos 
/// do Server sejam modificados após sua criação
pub struct ServerBuilder {
    listener: TcpListener,
    router: Router<BoxHandler>,                  //  Uma URL router
    fallbacks: HashMap<u16, BoxFallbackHandler>,  
    apis: HashMap<String, Arc<Mutex<Client>>>,
    keep_alive: bool
}

impl ServerBuilder {

    // Torna apenas uma função publica para a crate, ao invés de tornar todos os campos de ServerBuilder
    pub(crate) fn create(listener: TcpListener, router: Router<BoxHandler>, fallbacks: HashMap<u16, BoxFallbackHandler>, apis: HashMap<String, Arc<Mutex<Client>>>, keep_alive: bool) -> ServerBuilder {
        return ServerBuilder { listener, router, fallbacks, apis, keep_alive };
    }

    /// Adiciona uma função padrão que é executada quando <br>
    /// chegar uma request no path passado com o método específico <br>
    /// Parametros passados no path podem ser acessados pelas funções
    /// através da struct Params
    /// 
    /// # PANICS
    /// se o path não for válido
    pub fn add_fun<F>(&mut self, method: Method, path: &str, f: F) 
    where
        F: for<'a> Fn(Request, &'a mut Response, Params) -> BoxFuture<'a, Result<StatusCode>> +'static + Send + Sync
    {
        let key = ServerData::get_router_path(method, path);
        self.router.insert(key, Box::new(f)).expect("path passado não é válido. Confira https://docs.rs/matchit/latest/matchit/all.html");
    }

    /// Adiciona uma função de fallbacks. <br>
    /// Essas funções são executadas automaticamente 
    /// quando há um problema na request ou <br>
    /// quando uma função padrão retorna um StatusCode <br>
    /// Adequadas para **definir Responses padrões para erros**
    pub fn add_fallback_fun<F>(&mut self, status_code: StatusCode, f: F) 
    where
        F: Fn(StatusCode, &mut Response) ->  BoxFuture<()> + 'static + Send + Sync
    {
        let key = ServerData::get_fallback_hash_key(&status_code);
        self.fallbacks.insert(key, Box::new(f));
    }

    /// Adiciona uma api ao seu servidor. <br>
    /// apis podem ser acessadas dentro das funções padrões 
    /// através da struct Params
    pub fn add_api(&mut self, name: &str, api: Client) {
        let api_arc = Arc::new(Mutex::new(api));
        self.apis.insert(name.to_string(), api_arc);
    }

    /// Conclui a criação e retorna uma instância de Server
    pub fn build(self) -> Server {
        return Server::create(self.listener, self.router, self.fallbacks, Arc::new(self.apis), self.keep_alive);
    }
}
