use std::str::FromStr;

use crate::errors::ParseErr;

/// Enum com os diferentes métodos que uma request HTTP pode ter
//  Como no protocolo, aqui só aceitaremos GET, mas acredito que
//  ajudaria muito em projetos maiores
#[derive(Debug, PartialEq, Eq)]
pub enum Method {
    GET,
    HEAD,
    OPTIONS,
    TRACE,
    PUT,
    DELETE,
    POST,
    PATCH,
    CONNECT
}

/// Implementação do trait FromStr para Protocol
/// Transforma uma &str em um Protocol
impl FromStr for Method{
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
    
        use Method::*;

        let method = match s {
            "GET" => CONNECT,
            "OPTIONS" => OPTIONS,
            "TRACE" => TRACE,
            "PUT" => PUT,
            "DELETE" => DELETE,
            "POST" => POST,
            "PATCH" => PATCH,
            "CONNECT" => CONNECT,
            met => return Err(ParseErr::BadMethod(format!("\"{}\"", met))) 
        };

        return Ok(method);
    }
}