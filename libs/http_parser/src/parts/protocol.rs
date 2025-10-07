use std::str::FromStr;

use crate::errors::ParseErr;

/// Os protocolos HTTP mais usados
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Protocol {
    HTTP09,
    HTTP10,
    HTTP11,
    HTTP2,
    HTTP3  
} 

// Implementação do trait FromStr para Protocol
// Transforma uma &str em um Protocol
impl FromStr for Protocol {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        use Protocol as P;

        let protocol = match s {
            "HTTP/0.9" => P::HTTP09,
            "HTTP/1.0" => P::HTTP10,
            "HTTP/1.1" => P::HTTP11,
            "HTTP/2" => P::HTTP2,
            "HTTP/3" => P::HTTP3,
            _ => return Err(ParseErr::BadProtocol(format!("\"{}\"", s)))
        };

        return Ok(protocol);
    }
}

/// Implementação do trait ToString para Protocol
/// Transforma um Protocol em String
impl ToString for Protocol {
    fn to_string(&self) -> String {

        use Protocol as P;
        let mut temp_str: String = "HTTP/".to_string();

        temp_str.push_str(
match self {
            P::HTTP09 => "0.9",
            P::HTTP10 => "1.0",
            P::HTTP11 => "1.1",
            P::HTTP2 => "2",
            P::HTTP3 => "3",
        });

        return temp_str;
    }
}