use std::str::FromStr;

use crate::errors::ParseErr;

/// Enum com os diferentes métodos que uma request HTTP pode ter
//  Como no Methodo, aqui só aceitaremos GET, mas acredito que
//  ajudaria muito em projetos maiores
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

// Implementação do trait FromStr para Method
// Transforma uma &str em um Method
impl FromStr for Method{
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
    
        use Method as M;

        let method = match s {
            "GET" => M::GET,
            "HEAD" => M::HEAD,
            "OPTIONS" => M::OPTIONS,
            "TRACE" => M::TRACE,
            "PUT" => M::PUT,
            "DELETE" => M::DELETE,
            "POST" => M::POST,
            "PATCH" => M::PATCH,
            "CONNECT" => M::CONNECT,
            _ => return Err(ParseErr::BadMethod(format!("\"{}\"", s))) 
        };

        return Ok(method);
    }
}

// Implementação do trait ToString para Method
// Transforma um Method em String
impl ToString for Method {
    fn to_string(&self) -> String {
        use Method as M;

        return match self {
            M::GET => "GET",
            M::HEAD => "HEAD",
            M::OPTIONS => "OPTIONS",
            M::TRACE => "TRACE",
            M::PUT => "PUT",
            M::DELETE => "DELETE",
            M::POST => "POST",
            M::PATCH => "PATCH",
            M::CONNECT => "CONNECT",
        }.to_string();
    }
}