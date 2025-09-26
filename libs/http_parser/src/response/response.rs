
use std::str::FromStr;

use crate::{errors::ParseErr, response::response_builder::ResponseBuilder, Header, Protocol, StatusCode};

/// Struct para montagem modular de uma response HTTP
#[derive(Debug, Clone)]
pub struct Response {
    pub protocol: Protocol,
    pub status: StatusCode,
    pub headers: Header,
    pub body: Vec<u8>
}

/// Implementação do trait FromStr para Request
/// Transforma uma &str em um Request
impl TryFrom<&[u8]> for Response {
    type Error = ParseErr;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        //  É preciso remover os \r\n\r\n do final da request ou o reverse split falha
        let req = value.trim_ascii();

        //  Separa a request em seus elementos
        let (protocol, status_code, headers, body) = Response::get_elements(req)?;

        //  montagem da Request
        let http_req = Response {
            protocol: protocol,
            status: status_code,
            headers: headers,
            body: body,
        };

        return Ok(http_req);
    }
}

impl Response {

    /// Retorna uma instância de Response
    pub fn new() -> ResponseBuilder {
        return ResponseBuilder::new();
    }

    /// Converte uma Response em um vetor de bytes
    pub fn as_bytes(&self) -> Vec<u8> {
        //  Aloca com uma quantidade decente de memória para evitar realocações
        let mut vec = Vec::with_capacity(self.body.len() + 128);
        
        vec.append(&mut format!("{} {}\r\n",
                            self.protocol.to_string(),
                            self.status.to_string()).as_bytes().to_vec());
        
        if !self.headers.is_empty() {
            vec.append(&mut self.headers.to_string().as_bytes().to_vec());
        }
        
        vec.append(&mut b"\r\n".to_vec());
        
        if !self.body.is_empty() {
            vec.append(&mut self.body.clone());
            vec.append(&mut b"\r\n".to_vec());
        }
        
        return vec;
    }

    fn get_elements(response_bytes: &[u8]) -> Result<(Protocol, StatusCode, Header, Vec<u8>), ParseErr> {
        use ParseErr::*;

        let (others, body) = Response::split_body(response_bytes);

        let (main_header, header) = others.split_once("\r\n").unwrap_or((&others, ""));
        let mut parts = main_header.splitn(3, " ");

        //  separa o main header em método, path e protocolo
        let protocol = parts.next().ok_or(BadProtocol(others.clone()))?;
        let status_code = parts.next().ok_or(BadStatusCode(others.clone()))?;
        let status_message = parts.next().ok_or(BadStatusCode(others.clone()))?;
        return Ok((
                Protocol::from_str(protocol)?,
                StatusCode::from_str(status_code, Some(status_message))?,
                Header::from(header),
                body
        ));
    }

    fn split_body(response_bytes: &[u8]) -> (String, Vec<u8>) {
        
        let index = Self::find_body_index(response_bytes);
        
        return match index {
            Some(i) => {
                let (other, body) = response_bytes.split_at(i);
                (String::from_utf8_lossy(other).to_string(), Vec::from(body))
            }
            None => (String::from_utf8_lossy(response_bytes).to_string(), Vec::new()),
        }
    
    }

    fn find_body_index(response_bytes: &[u8]) -> Option<usize> {
        for i in 0..response_bytes.len() {
            if response_bytes[i..].starts_with(b"\r\n\r\n") {return Some(i);}
        }
        return None;
    }
 
}

    