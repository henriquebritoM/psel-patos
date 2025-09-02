use std::str::FromStr;

use crate::errors::ParseErr;

/// Enum com os diferentes métodos que uma request HTTP pode ter
//  Como no Methodo, aqui só aceitaremos GET, mas acredito que
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

/// Implementação do trait FromStr para Method
/// Transforma uma &str em um Method
impl FromStr for Method{
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
    
        use Method::*;

        let method = match s {
            "GET" => GET,
            "HEAD" => HEAD,
            "OPTIONS" => OPTIONS,
            "TRACE" => TRACE,
            "PUT" => PUT,
            "DELETE" => DELETE,
            "POST" => POST,
            "PATCH" => PATCH,
            "CONNECT" => CONNECT,
            _ => return Err(ParseErr::BadMethod(format!("\"{}\"", s))) 
        };

        return Ok(method);
    }
}

/// Implementação do trait ToString para Method
/// Transforma um Method em String
impl ToString for Method {
    fn to_string(&self) -> String {
        use Method::*;

        return match self {
            GET => "GET",
            HEAD => "HEAD",
            OPTIONS => "OPTIONS",
            TRACE => "TRACE",
            PUT => "PUT",
            DELETE => "DELETE",
            POST => "POST",
            PATCH => "PATCH",
            CONNECT => "CONNECT",
        }.to_string();
    }
}