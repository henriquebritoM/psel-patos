use std::str::FromStr;

use crate::errors::ParseErr;
use crate::parsers::{header_parser::Header, method_parser::Method, protocol_parser::Protocol};

/// Struct para desmembrar uma request http padrão
///
#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub protocol: Protocol,
    pub headers: Vec<String>,
    pub body: String,
}

/// Implementação do trait FromStr para Request
/// Transforma uma &str em um Request
impl FromStr for Request {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //  É preciso remover os \r\n\r\n do final da request ou o reverse split falha
        let req = s.trim();

        //  Separa a request em seus elementos
        let (method, path, protocol, headers, body) = Request::get_elements(req)?;

        //  montagem da Request
        let http_req = Request {
            method: Method::from_str(&method)?,
            path: path.to_string(),
            protocol: Protocol::from_str(&protocol)?,
            headers: Header::get_vec(&headers),
            body: body.to_string(),
        };

        return Ok(http_req);
    }
}

impl Request {
    ///  Separa um request http em seus componentes
    ///  retorna um ParseErr se não for possível
    fn get_elements(request: &str) -> Result<(String, String, String, String, String), ParseErr> {
        use ParseErr::*;

        //  Separa o body do resto da request
        //  em seguida separa o resto em main header e headers
        let (req_and_head, body) = request
            .rsplit_once("\r\n\r\n")
            .unwrap_or( (request, "") );
        let (main_header, headers) = req_and_head
            .split_once("\r\n")
            .unwrap_or((req_and_head, "") );

        let mut parts = main_header.splitn(4, " ");

        //  separa o main header em método, path e protocolo
        let method = parts.next().ok_or(BadMethod(request.to_string()))?;
        let mut path = parts.next().ok_or(BadPath(request.to_string()))?;
        let protocol = parts.next().ok_or(BadProtocol(request.to_string()))?;

        //  evitar problemas, path base
        if path.is_empty() {
            path = "/";
        }

        return Ok((
            method.to_string(),
            path.to_string(),
            protocol.to_string(),
            headers.to_string(),
            body.to_string(),
        ));
    }
}
