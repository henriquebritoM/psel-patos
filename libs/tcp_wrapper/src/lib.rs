use std::{io::{ErrorKind, Read, Write}, net::TcpStream};

use http_parser::{ParseErr, Request, Response};

/// Lê a stream e converte o resultado para Request <br>
/// utiliza o header content-length para ler o body <br>
/// **sem content-length = sem leitura do body**
/// None significa que o outro lado desconectou
pub fn read_request(stream: &mut TcpStream) -> Option<Result<Request, ParseErr>> {

    let mut vec: Vec<u8> = Vec::with_capacity(4096);

    vec.append(&mut read_until_body(stream)? );

    let mut req = match Request::try_from(vec.as_slice()) {
        Ok(r) => r,
        Err(e) => return Some(Err(e)),
    };

    let body_len = req.headers.get_header_value("Content-Length").unwrap_or("0".to_string());
    let body_len: usize = body_len.parse().unwrap_or(0);
    
    req.body = read_body(stream, body_len);

    return Some(Ok(req));
}

/// Lê a stream e converte o resultado para Response <br>
/// utiliza o header content-length para ler o body <br>
/// **sem content-length = sem leitura do body**
pub fn read_response(stream: &mut TcpStream) -> Option<Result<Response, ParseErr>> {

    let mut vec: Vec<u8> = Vec::with_capacity(4096);

    vec.append(&mut read_until_body(stream)?);

    let mut res = match Response::try_from(vec.as_slice()) {
        Ok(r) => r,
        Err(e) => return Some(Err(e)),
    };

    let body_len = res.headers.get_header_value("Content-Length").unwrap_or("0".to_string());
    let body_len: usize = body_len.parse().unwrap_or(0);
    
    res.body = read_body(stream, body_len);

    return Some(Ok(res));
}

/// Lê a stream até encontar **\r\n\r\n** <br>
/// se não encontrar retorna o lido <br>
/// # Atenção
/// pode ficar preso em loop se a stream não se desconectar nem tiver \r\n\r\n <br>
/// solução: adicione um **stream.read_timeout()** antes de chamar a função
fn read_until_body(stream: &mut TcpStream) -> Option<Vec<u8>> {
    
    let mut vec: Vec<u8> = Vec::new();
    let mut buffer: [u8; 1] = [0; 1];
    
    loop {

        match stream.read(&mut buffer) {
            Ok(0) => return None,                                                           //  EOF sai do loop
            Ok(_) => {},                                                                    //  Byte lido, continua execuçãos
            Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,           //  Tenta novamente
            Err(_) => {return None;}                                                 //  Erro irrecuperável, retorna
        };

        vec.push(buffer[0]);                                                                 //  Coloca o byte lido no vec
        if vec.ends_with(b"\r\n\r\n") {return Some(vec);}                            //  para de ler e retorna se chegar no body
    }                                                           
}

/// Lê exatamente o número de bytes lidos,
/// se não for possível retorna um vetor vazio
fn read_body(stream: &mut TcpStream, len: usize) -> Vec<u8> {
    let mut vec: Vec<u8> = vec![0; len];

    match stream.read_exact(&mut vec) {
        Err(_) => {vec.clear(); vec.shrink_to_fit();}       //  limpa o vetor o trunca. vec deve ficar vazio e usar a memória mínima
        Ok(_) => {},                                        //  Tudo certo, vetor foi preenchido como esperado
    };

    return vec;
}

/// Envia um &[u8] pela stream,
/// um erro indica que provavelmente o destinatário se desconectou
pub fn write_stream(stream: &mut TcpStream, bytes: &[u8]) -> Result<(), ErrorKind> {
    loop {
        match stream.write_all(bytes) {
            Ok(_) => return Ok(()),
            Err(e) => return Err(e.kind()),
        }
    }
}


