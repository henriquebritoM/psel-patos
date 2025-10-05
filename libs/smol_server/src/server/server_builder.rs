use std::collections::HashMap;
use std::sync::Arc;

use http_parser::{Request, Response};
use http_parser::{Method, StatusCode};
use matchit::{Router};
use tokio::net::TcpListener;
use tokio::sync::Mutex;

use crate::server::server_data::ServerData;
use crate::{server::fn_handler::{BoxFallbackHandler, BoxHandler}, Client, Params, Server};

/// struct para construção de um Server
/// Esse modele de construtor impede que os campos 
/// do Server sejam modificados após sua criação
pub struct ServerBuilder {
    listener: TcpListener,
    router: Router<(BoxHandler, Params)>,                  //  Uma URL router
    fallbacks: HashMap<u16, BoxFallbackHandler>,  
    apis: HashMap<String, Arc<Mutex<Client>>>,
    temp_fns: Vec<(String, BoxHandler)>               //  só é possível colocar no router durante o build
}

impl ServerBuilder {

    // Torna apenas uma função publica para a crate, ao invés de tornar todos os campos de ServerBuilder
    pub(crate) fn create(listener: TcpListener) -> ServerBuilder {
        return ServerBuilder { listener, router: Router::new(), fallbacks: HashMap::new(), apis: HashMap::new(), temp_fns: Vec::new() };
    }

    /// Adiciona uma função padrão que é executada quando <br>
    /// chegar uma request no path passado com o método específico <br>
    /// Parametros passados no path podem ser acessados pelas funções
    /// através da struct Params
    /// 
    /// # PANICS
    /// se o path não for válido
    pub fn add_fun<F, Fut>(&mut self, method: Method, path: &str, f: F) 
    where
         //BoxFuture<'a, Result<StatusCode>> +'static + Send + Sync
        F: Fn(Request, Response, Params) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Response, StatusCode>> + Send + Sync + 'static
    {
        let key = ServerData::get_router_path(method, path);
        // let future = f()
        self.temp_fns.push((key, Box::new(f)));  
    }

    /// Adiciona uma função de fallbacks. <br>
    /// Essas funções são executadas automaticamente 
    /// quando há um problema na request ou <br>
    /// quando uma função padrão retorna um StatusCode <br>
    /// Adequadas para **definir Responses padrões para erros**
    pub fn add_fallback_fun<F, Fut>(&mut self, status_code: StatusCode, f: F) 
    where
        F: Fn() ->  Fut + 'static + Send + Sync,
        Fut: Future<Output = Response> + Send + Sync + 'static
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
    pub fn build(mut self) -> Server {
        let apis_arc = Arc::new(self.apis); //  A partir de agora isso não pode mais ser modificado
        let p = Params{ apis: apis_arc.clone(), arguments: HashMap::new() };    

        for (key, f) in self.temp_fns {
            self.router.insert(key, (f, p.clone())).unwrap(); //    Mandar a mensagem de erro do matchit
        };
        return Server::create(self.listener, self.router, self.fallbacks, apis_arc);
    }
}

