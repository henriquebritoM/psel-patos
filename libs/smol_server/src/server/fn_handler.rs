//  Esse código foi FORTEMENTE inspirado na implementação do Axum
//  eles tiraram leite de pedra para criar isso

use http_parser::{Request, Response, StatusCode};
use futures::future::BoxFuture;

use crate::Params;

pub type Result<StatusCode> = std::result::Result<(), StatusCode>;
pub type BoxHandler = Box<dyn FnHandler + Send>;
pub type BoxFallbackHandler = Box<dyn FallbackHandler + Send>;

pub trait FnHandler: Send + Sync{
    /// self: Referência para a função que implementa o trait
    /// req: a request em questão
    /// res: referência exclusiva à response
    /// 'a o futuro só é válido enquanto houver a response
    fn call<'a>(&self, req: Request, res: &'a mut Response, params: Params) -> BoxFuture<'a, Result<StatusCode>>;
}

impl<F> FnHandler for F
where
    F: for<'a> Fn(Request, &'a mut Response, Params) -> BoxFuture<'a, Result<StatusCode>> + 'static + Send + Sync,
{
    fn call<'a>(&self, req: Request, res: &'a mut Response, params: Params) -> BoxFuture<'a, Result<StatusCode>>
    {
        self(req, res, params)
    }
}

pub trait FallbackHandler: Send + Sync {
    /// self: Referência para a função que implementa o trait
    /// req: a request em questão
    /// res: referência exclusiva à response
    /// 'a o futuro só é válido enquanto houver a response
    fn call<'a>(&self, status: StatusCode, res: &'a mut Response) -> BoxFuture<'a, ()>;    
}

impl<F> FallbackHandler for F 
where
    F: Fn(StatusCode, &mut Response) ->  BoxFuture<()> + 'static + Send + Sync
{
    /// Executa a função passada e retorna seu Futuro
    //  Não podemos declarar funções async dentro de traits,
    //  então o futuro terá que ser await na implementação
    fn call<'a>(&self, status: StatusCode, res: &'a mut Response) -> BoxFuture<'a, ()> {
        self(status, res)
    }
}
