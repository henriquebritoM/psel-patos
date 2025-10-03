use std::str::FromStr;

use crate::errors::{ParseErr};
use crate::{Header, Method, Protocol};

/// Struct para desmembrar uma request http padrão
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub protocol: Protocol,
    pub headers: Header,
    pub body: Vec<u8>,      //  Um vetor de u8, pois pode conter dados em binário, não só em UTF-8
}

/// Implementação do trait FromStr para Request
/// Transforma uma &str em um Request
impl TryFrom<&[u8]> for Request {
    type Error = ParseErr;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        //  É preciso remover os \r\n\r\n do final da request ou o reverse split falha
        let req = value.trim_ascii();

        //  Separa a request em seus elementos
        let (method, path, protocol, headers, body) = Request::get_elements(req)?;

        //  montagem da Request
        let http_req = Request {
            method,
            path,
            protocol,
            headers,
            body,
        };

        return Ok(http_req);
    }
}

impl Request {
    pub fn new() -> Request {
        return Request { 
            method: Method::GET,
            path: "/".to_string(),
            protocol:Protocol::Http11,
            headers: Header::new(),
            body: Vec::new()
        }
    }

    pub fn method(mut self, m: Method) -> Request {
        self.method = m;
        return self;
    }

    pub fn path(mut self, p: &str) -> Request {
        self.path = p.to_string();
        return self;
    }

    pub fn protocol(mut self, p: Protocol) -> Request {
        self.protocol = p;
        return self;
    }

    pub fn add_header<T: ToString, U: ToString>(mut self, field: T, value: U) -> Request {
        self.headers.add_header(field, value);
        return self;
    } 

    pub fn body<T: Into<Vec<u8>>>(mut self, body: T) -> Request {
        self.body = body.into();
        let len = self.body.len();
        self.add_header("Content-Length", len)
    }
}

impl Request {

    /// Converte uma Request para bytes
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::with_capacity(self.body.len() + 128);
        
        //  Push no cabeçalho principal
        vec.append(&mut format!("{} {} {}\r\n", self.method.to_string(), self.path, self.protocol.to_string()).as_bytes().to_vec());
        
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

    ///  Separa um request http em seus componentes
    ///  retorna um ParseErr se não for possível
    fn get_elements(request_bytes: &[u8]) -> Result<(Method, String, Protocol, Header, Vec<u8>), ParseErr> {
        use ParseErr::*;

        //  Separa o body do resto da request, se não houver body, ele é inicializado vazio
        let (others, body) = Self::split_body(request_bytes);

        //  Separa os headers do cabeçalho principal
        let (main_header, header) = others
            .split_once("\r\n")
            .unwrap_or((&others, "") );

        //  Separa as 4 partes do cabeçalho principal: Método, Path, Protocolo 
        let mut parts = main_header.splitn(3, " ");

        //  separa o main header em método, path e protocolo
        let method = parts.next().ok_or(BadMethod(others.clone()))?;
        let mut path = parts.next().ok_or(BadPath(others.clone()))?;
        let protocol = parts.next().ok_or(BadProtocol(others.clone()))?;

        //  evitar problemas, path base
        if path.is_empty() {
            path = "/";
        }

        return Ok((
            Method::from_str(method)?,
            path.to_string(),
            Protocol::from_str(protocol)?,
            Header::from(header),
            body,
        ));
    }
    
    fn split_body(request_bytes: &[u8]) -> (String, Vec<u8>) {
        
        let index = Self::find_body_index(request_bytes);
        
        return match index {
            Some(i) => {
                let (other, body) = request_bytes.split_at(i);
                (String::from_utf8_lossy(other).to_string(), Vec::from(body))
            }
            None => (String::from_utf8_lossy(request_bytes).to_string(), Vec::new()),
        }
    
    }

    fn find_body_index(request_bytes: &[u8]) -> Option<usize> {
        for i in 0..request_bytes.len() {
            if request_bytes[i..].starts_with(b"\r\n\r\n") {return Some(i);}
        }
        return None;
    }

}

