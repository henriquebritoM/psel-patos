//  Esse código foi FORTEMENTE inspirado na implementação do Axum
//  eles tiraram leite de pedra para criar isso

use http_parser::{Request, Response, StatusCode};
use futures::future::{BoxFuture, FutureExt};

use crate::Params;

/*
 *  "Não é possível" passar um ponteiro para uma função que retorne um Future <br>
 *  Os traits FnHandler e FallbackHandler permitem contornam essa limitação,
 *  funcionando como wrappers em torno das funções originais, podendo ser passados
 *  como "trait objects"
 *  É preciso dar .await nos Futures retornados por call(), ou não haverá 
 *  execução (como qualquer Future)
*/

/// Um trait object de FnHandler
pub type BoxHandler = &'static (dyn FnHandler);

/// Um trait object de FallbackHandler
pub type BoxFallbackHandler = &'static (dyn FallbackHandler);

/// Trait implementado para funções com a seguinte assinatura:
/// 
/// # Exemplos
/// 
/// ```rust 
/// # use http_parser::{Request, Response, StatusCode};
/// # use smol_server::{Params, FnHandler};
/// # fn is_fn_hanlder<F: FnHandler>(f: F) -> bool {true};
/// 
/// async fn foo(req: Request, res: Response, p: Params) -> Result<Response, StatusCode> {
///     Err(StatusCode::NotFound)
/// }
/// 
/// assert_eq!(is_fn_hanlder(foo), true);
/// ```
pub trait FnHandler: Send + Sync {

    /// "Chama" a função
    /// é necessário usar o .await no BoxFuture, ou ela não executará
    fn call(&'_ self, req: Request, res: Response, params: Params) -> BoxFuture<'_, Result<Response, StatusCode>>;
}

impl<F, Fut> FnHandler for F
where
    F: Fn(Request, Response, Params) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<Response, StatusCode>> + Send + Sync + 'static
{
    /// Executa a função passada e retorna seu Futuro
    //  Não podemos declarar funções async dentro de traits,
    //  então é preciso dar .await no BoxFuture
    fn call(&'_ self, req: Request, res: Response, params: Params) -> BoxFuture<'_, Result<Response, StatusCode>> {
        self(req, res, params).boxed()
    }
}

/// Trait implementado para funções com a seguinte assinatura:
/// # Exemplos
/// 
/// ```rust 
/// # use smol_server::FallbackHandler;
/// # use http_parser::{StatusCode, Response};
/// # fn is_fallback_hanlder<F: FallbackHandler>(f: F) -> bool {true};
/// 
/// async fn foo() -> Response {
///     Response::default(StatusCode::NotFound)
/// }
/// 
/// assert_eq!(is_fallback_hanlder(foo), true);
/// ```
pub trait FallbackHandler: Send + Sync {
    
    /// "Chama" a função
    /// é necessário usar o .await no BoxFuture, ou ela não executará
    fn call(&'_ self) -> BoxFuture<'_, Response>;    
}

impl<F, Fut> FallbackHandler for F 
where
    F: Fn() ->  Fut + 'static + Send + Sync,
    Fut: Future<Output = Response> + Send + Sync + 'static
{
    /// Executa a função passada e retorna seu Futuro
    //  Não podemos declarar funções async dentro de traits,
    //  então é preciso dar .await no BoxFuture
    fn call(&'_ self) -> BoxFuture<'_, Response> {
        self().boxed()
    }
}
