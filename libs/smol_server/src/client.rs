use tokio::net::{TcpStream, ToSocketAddrs};

use http_parser::{Request, Response, StatusCode};
use tcp_wrapper::{read_response, write_stream};

/// Um handler para uma conexão com algum servidor
pub struct Client {
    server_stream: TcpStream,
}

impl Client {
    /// Conecta-se ao addr passado, None se não for possível
    pub async fn init<A: ToSocketAddrs>(addr: A) -> Option<Client> {
        let stream = TcpStream::connect(addr).await.ok()?;

        return Some(Client {server_stream: stream });
    }

    /// Tenta se reconectar ao servidor
    async fn _try_reconnect(&mut self) -> bool {
        let Ok(addr) = self.server_stream.peer_addr() else {return false;};

        let Ok(new_stream) = TcpStream::connect(addr).await else {return false;};

        self.server_stream = new_stream;
        return true;
    }

    /// Retorna a stream que Client está ligado
    pub fn get_stream(&mut self) -> &mut TcpStream {
        return &mut self.server_stream
    }

    /// Envia uma Request para o servidor
    pub async fn send(&mut self, request: &Request) -> bool {
        write_stream(&mut self.server_stream, &request.as_bytes()).await.is_ok()
    }

    /// Recebe uma Response do servidro
    pub async fn receive(&mut self) -> Response {
        let response = read_response(&mut self.server_stream).await; //.expect("api se desconectou");
        return match response {
            Some(Ok(r)) => r,
            _ => Response::default(StatusCode::InternalServerError),
        }
    }

    /// Envia uma Request ao servidor e retorna a Response
    pub async fn fetch(&mut self, request: &Request) -> Response {
        self.send(request).await;
        return self.receive().await;
    }
}