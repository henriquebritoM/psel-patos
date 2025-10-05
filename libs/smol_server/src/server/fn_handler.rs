//  Esse código foi FORTEMENTE inspirado na implementação do Axum
//  eles tiraram leite de pedra para criar isso

use http_parser::{Request, Response, StatusCode};
use futures::future::{BoxFuture, FutureExt};

use crate::Params;

// pub type Result<StatusCode> = std::result::Result<Response, StatusCode>;
pub type BoxHandler = Box<dyn FnHandler + Send>;
pub type BoxFallbackHandler = Box<dyn FallbackHandler + Send>;

pub trait FnHandler: Send + Sync{
    /// self: Referência para a função que implementa o trait
    /// req: a request em questão
    /// res: referência exclusiva à response
    /// 'a o futuro só é válido enquanto houver a response
    fn call(&'_ self, req: Request, res: Response, params: Params) -> BoxFuture<'_, Result<Response, StatusCode>>;
}

impl<F, Fut> FnHandler for F
where
    F: Fn(Request, Response, Params) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<Response, StatusCode>> + Send + Sync + 'static
    // F: for<'a> Fn(Request, &'a mut Response, Params) -> BoxFuture<'a, Result<StatusCode>> + 'static + Send + Sync,
{
    fn call(&'_ self, req: Request, res: Response, params: Params) -> BoxFuture<'_, Result<Response, StatusCode>> {
        self(req, res, params).boxed()
    }
}

pub trait FallbackHandler: Send + Sync {
    /// self: Referência para a função que implementa o trait
    /// req: a request em questão
    /// res: referência exclusiva à response
    /// 'a o futuro só é válido enquanto houver a response
    fn call(&'_ self) -> BoxFuture<'_, Response>;    
}

impl<F, Fut> FallbackHandler for F 
where
    F: Fn() ->  Fut + 'static + Send + Sync,
    Fut: Future<Output = Response> + Send + Sync + 'static
{
    /// Executa a função passada e retorna seu Futuro
    //  Não podemos declarar funções async dentro de traits,
    //  então o futuro terá que ser await na implementação
    fn call(&'_ self) -> BoxFuture<'_, Response> {
        self().boxed()
    }
}
