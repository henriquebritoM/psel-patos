use std::net::{TcpStream, ToSocketAddrs};

use http_parser::{Request, Response, StatusCode};
use tcp_wrapper::{read_response, write_stream};

pub struct Client {
    server_stream: TcpStream,
}

impl Client {
    /// Conecta-se ao addr passado, None se não for possível
    pub fn init<A: ToSocketAddrs>(addr: A) -> Option<Client> {
        let stream = TcpStream::connect(addr).ok()?;

        return Some(Client {server_stream: stream });
    }

    fn _try_reconnect(&mut self) -> bool {
        let Ok(addr) = self.server_stream.peer_addr() else {return false;};

        let Ok(new_stream) = TcpStream::connect(addr) else {return false;};

        self.server_stream = new_stream;
        return true;
    }

    pub fn get_stream(&mut self) -> &mut TcpStream {
        return &mut self.server_stream
    }

    pub fn send(&mut self, request: &mut Request) -> bool {
        write_stream(&mut self.server_stream, &request.as_bytes()).is_ok()
    }

    pub fn receive(&mut self) -> Response {
        let response = read_response(&mut self.server_stream).expect("api se desconectou");
        return match response {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Erro ao receber response: {e}");
                Response::new().status(StatusCode::InternalServerError).build()
            }
        }
    }
}