use std::str::FromStr;

use crate::errors::ParseErr;

/// Enum com os diferentes procolos que uma mensagem http pode ter
//  aqui só vamos usar http/1.1, mas teria mais utilidade  
//  se fosse preciso uma peneira mais fina
#[derive(Debug)]
pub enum Protocol {
    Http09,
    Http10,
    Http11,
    Http2,
    Http3  
} 

/// Implementação do trait FromStr para Protocol
/// Transforma uma &str em um Protocol
impl FromStr for Protocol {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        use Protocol::*;

        let protocol = match s {
            "HTTP/0.9" => Http09,
            "HTTP/1.0" => Http10,
            "HTTP/1.1" => Http11,
            "HTTP/2" => Http2,
            "HTTP/3" => Http3,
            _ => return Err(ParseErr::BadProtocol(format!("\"{}\"", s)))
        };

        return Ok(protocol);
    }
}