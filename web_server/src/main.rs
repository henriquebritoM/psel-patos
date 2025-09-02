use std::{backtrace::BacktraceStatus, io::{BufRead, BufReader, ErrorKind, Read}, str::FromStr};
use http_parser::request::Request;
use std::net::{TcpListener, TcpStream};


fn main() {
    println!("starting reverse proxy");

    //  listener para as conexões dos clientes
    // let listener = TcpListener::bind("localhost:0").expect("Não foi possível se conectar a nenhuma porta");
    let listener = TcpListener::bind("localhost:8080").expect("Não foi possível se conectar a nenhuma porta");

    println!("Escutando na porta {}", listener.local_addr().unwrap());

    //  Loop principal das conexões dos clientes
    for stream in listener.incoming() {

        //  Ignora as coneções que falharam em se conectar
        let stream = match stream {
            Ok(s) => s,
            Err(_) => continue,
        };

        let str = read_stream(&stream).unwrap();
        println!("{:?}", str);
        let request = Request::from_str(&str).unwrap();

        println!("{:?}", request);

    }

}

/// Retorna uma String lida da stream
/// None indica que o cliente se desconectou, recomendado fechar a stream
fn read_stream(mut stream: &TcpStream) -> Option<String> {

    let mut temp_str = String::new();
    let mut buffer: [u8; 4096] = [b' '; 4096];

    loop {
        match stream.read(&mut buffer) {
            Ok(_) => break,
            Err(e) if e.kind() == ErrorKind::Interrupted => continue,
            _ => return None
        }
    }
    
    temp_str.push_str(&String::from_utf8_lossy(&buffer));

    return Some(temp_str.trim().to_string());
}
