use std::{collections::HashMap, sync::Arc};

use http_parser::{Method, Request, Response, StatusCode};
use matchit::Router;
use tokio::sync::Mutex;

use crate::{server::fn_handler::{BoxFallbackHandler, BoxHandler}, Client};


/// Guarda dados gerais que diferentes threads podem querer
/// acessar ao gerar uma response
pub struct ServerData {
    router: Router<(BoxHandler, Params)>,                  //  Uma URL router
    fallbacks: HashMap<u16, BoxFallbackHandler>,
}

impl ServerData {
    // Torna apenas uma função publica para a crate, ao invés de tornar todos os campos de ServerData
    pub(crate) fn create(router: Router<(BoxHandler, Params)>, fallbacks: HashMap<u16, BoxFallbackHandler>) -> &'static ServerData {
        let data = ServerData {router, fallbacks};
        return  data.to_static();
    }

    pub(crate) fn get_func(&self, key: &str) -> Option<(BoxHandler, Params)> {
        let Ok(matched) = self.router.at(key) else {return None;};

        let (f, p) = matched.value;
        let mut p = p.clone();  // Os parâmetros são individuais para cada request, clonar é valido nessa situação

        let _ = matched.params.iter().map(
            |(key, value)| p.arguments.insert(key.to_string(), value.to_string())
        );

        return Some((*f, p));
    }

    pub(crate) fn get_fallback_func(&self, key: u16) -> Option<&BoxFallbackHandler>  {
        self.fallbacks.get(&key)
    }

    //  PORQUE um ' ' (whitespace) separando os campos?
    //  durante um parsing de Request/Response um ' ' no meio do path
    //  resultaria em um erro. Portanto não é possível acessar keys arbitrárias com o path.
    /// converte o método path passado para router path
    pub(crate) fn get_router_path(method: Method, path: &str) -> String {
        return method.to_string() + " " + path;
    }

    /// Converte um StatusCode em uma key 
    pub(crate) fn get_fallback_hash_key(status_code: &StatusCode) -> u16 {
        return status_code.get_code();
    }
}

#[derive(Clone)]
pub struct Params {
    pub apis: &'static HashMap<String, Arc<Mutex<Client>>>,
    pub arguments: HashMap<String, String>
}

impl Params {
    pub async fn api_fetch(&self, api_name: &str, mut req: Request) -> Response {
        let Some(api_temp) = self.apis.get(api_name) else {
            return Response::new().status(StatusCode::InternalServerError).build();
        };
        let mut api  = api_temp.lock().await;
        api.send(&mut req).await;
        return api.receive().await;
    }

    pub fn get(&self, param_name: &str) -> String {
        return self.arguments.get(param_name).expect("\'{}\' não é um parâmetro").to_string();
    }
}

/// Causa um **memory leak** no dado passado, <br>
/// o tornando válido por toda a duração do programa
/// 
/// # NÃO ABUSE
pub(crate) trait ToStatic: Sized {
    fn to_static(self) -> &'static Self {
        let boxed = Box::new(self);
        let leaked: &'static Self = Box::leak(boxed);
        return leaked;
    }
}

impl<T: Sized> ToStatic for T {}