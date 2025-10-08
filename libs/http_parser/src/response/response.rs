
use std::str::FromStr;

use crate::{errors::ParseErr, Header, Protocol, StatusCode};

/// Struct que permite parsing, acesso e manipulação de 
/// responses http
#[derive(Debug, Clone)]
pub struct Response {
    pub protocol: Protocol,
    pub status: StatusCode,
    pub headers: Header,
    pub body: Vec<u8>
}

// Implementação do trait FromStr para Request
// Transforma uma &str em um Request
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

//  Setters e Getters
impl Response {

    /// Retorna nova instância de Response
    /// por padrão inicializada como:
    /// "HTTP/1.1 404 Not Found"
    pub fn new() -> Response {
        let mut r = Response {
            protocol: Protocol::HTTP11,
            status: StatusCode::NotFound,
            headers: Header::new(),
            body: Vec::new(),
        };
        
        r.add_header("Content-Length", 0);
        r
    }

    /// Shorthand para fechar a conexão. <br>
    /// Streams, por padrão, ficam abertar até que o client
    /// explicite o fechamento
    pub fn close(&mut self) {   
        self.headers.add_header("Connection", "close");
    }

    /// retorna Header: Connection == close
    pub fn closing(&self) -> bool {
        return self.headers.get_header("Connection") == Some("close".to_owned());
    }

    /// Seta um protocolo para a Response   
    pub fn protocol(&mut self, p: Protocol) -> &mut Response {
        self.protocol = p;
        return self;
    }

    /// Seta um status code para a Response
    pub fn status(&mut self, sc: StatusCode) -> &mut Response {
        self.status = sc;
        return self;
    }

    /// Adiciona um header à Response
    pub fn add_header<T: ToString, U: ToString>(&mut self, field: T, value: U) -> &mut Response {
        self.headers.add_header(field, value);
        return self;
    } 

    /// Seta um body para a Response <br>
    /// automaticamente adiciona o header
    /// "Content-Length"
    pub fn body<T: Into<Vec<u8>>>(&mut self, body: T) -> &mut Response {
        self.body = body.into();
        let len = self.body.len();
        self.add_header("Content-Length", len);
        return self;
    }

    //  É necessário clonar o valor, mesma approach que o pessoal 
    //  do "derive_builder", segundo eles a perda de performance é ínfima
    /// Controi uma instância de Response   <br>
    /// automaticamente inclui o header "Content-Length"
    pub fn build (&mut self) -> Response {
        let len = self.body.len();
        self.add_header("Content-Length", len);
        return self.clone();    
    }

    /// Converte uma Response em um Vec<u8> (Vetor de bytes)
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
            // vec.append(&mut b"\r\n\r\n".to_vec());
        }
        
        return vec;
    }
    
    /// Retorna uma response padrão para o a status code passado
    /// uma response padrão se parece com isso:
    /// ```html
    /// <!DOCTYPE html>
    /// <html lang="en">
    /// <head>
    ///     <meta charset="UTF-8">
    ///     <meta name="viewport" content="width=device-width, initial-scale=1.0">
    ///     <title>Error 400</title>
    /// </head>
    /// <body>
    ///     <h1>Error 400 - Bad request</h1>
    /// </body>
    /// </html>
    /// ```
    pub fn default(sc: StatusCode) -> Response {

        let l1: &'static str = r#"<!DOCTYPE html>"#;
        let l2: &'static str = r#"<html lang="en">"#;
        let l3: &'static str = r#"<head>"#;
        let l4: &'static str = r#"    <meta charset="UTF-8">"#;
        let l5: &'static str = r#"    <meta name="viewport" content="width=device-width, initial-scale=1.0">"#;
        let l6: &str = &format!("    <title> Error {} </title>", sc.get_code());
        let l7: &'static str = r#"</head>"#;
        let l8: &'static str = r#"<body>"#;
        let l9: &str = &format!("    <h1> Error {} - {} </h1>", sc.get_code(), sc.get_text());
        let l10: &'static str= r#"</body>"#;
        let l11: &'static str= r#"</html>"#;

        let concat: &str = &format!("{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}", l1, l2, l3, l4, l5, l6, l7, l8, l9, l10, l11);

        let mut response: Response = 
            Response::new()
            .status(sc)
            .body(concat.as_bytes())
            .build();

        response.close();
        return response;
        
    }

}



//  Helpers privados
impl Response {

    /// Faz o parsing de uma slice de bytes
    /// retorna um ParseErr se não for possível
    fn get_elements(response_bytes: &[u8]) -> Result<(Protocol, StatusCode, Header, Vec<u8>), ParseErr> {
        use ParseErr::*;

        let (others, body) = Response::split_body(response_bytes);

        let (main_header, header) = others.split_once("\r\n").unwrap_or((&others, ""));
        let mut parts = main_header.splitn(3, " ");

        //  separa o main header em método, path e protocolo
        let protocol = parts.next().ok_or(BadProtocol(others.clone()))?;
        let status_code = parts.next().ok_or(BadStatusCode(others.clone()))?;
        let _status_message = parts.next().ok_or(BadStatusCode(others.clone()))?;   //  Seria mais útil se houvesse status code customizáveis
        return Ok((
                Protocol::from_str(protocol)?,
                StatusCode::from_str(status_code)?,
                Header::from(header),
                body
        ));
    }

    ///  Separa o body do resto da request, se não houver body, ele é inicializado vazio
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

    /// Encontra o índice do body na request
    fn find_body_index(response_bytes: &[u8]) -> Option<usize> {
        for i in 0..response_bytes.len() {
            if response_bytes[i..].starts_with(b"\r\n\r\n") {return Some(i+4);}
        }
        return None;
    }
 
}

    