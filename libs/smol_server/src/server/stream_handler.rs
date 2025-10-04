use std::sync::Arc;
use http_parser::{Request, Response, StatusCode};
use tcp_wrapper::{read_request, write_stream};
use tokio::{net::TcpStream};

use crate::server::server_data::ServerData;

pub(crate) struct ConectionHandler {
    data: Arc<ServerData>,
    stream: TcpStream
}

impl ConectionHandler {
    pub(crate) async fn handle(data: Arc<ServerData>, stream: TcpStream) {
        let mut handler = ConectionHandler { data, stream };
        handler.handle_connection().await;
    }

    pub(crate) async fn handle_connection(&mut self) {
        // client_stream.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
        // client_stream.set_write_timeout(Some(Duration::from_secs(5))).unwrap();
        let Some(request) = read_request(&mut self.stream).await else {return;};    //  Outro lado se desconectou, aborto

        let response: Response;

        if let Ok(r) = request {
            response = self.get_response(r).await
        } else {
            eprintln!("Request fora de formato, rodando fallback BadRequest");
            response = self.run_fallback(StatusCode::BadRequest).await;
        }
        
        let Ok(_) = write_stream(&mut self.stream, &response.as_bytes()).await else {return;};    //  Outro lado se desconectou, aborto
    }
    

    pub(crate) async fn get_response(&mut self, request: Request) -> Response {

        let key = ServerData::get_router_path(request.method, &request.path);
        let Some((func, params)) = self.data.get_func(&key) else {
            eprintln!("Função não encontrada, rodando fallback");
            return self.run_fallback(StatusCode::NotFound).await;
        };

        let mut response = Response::new();

        if let Err(sc) = func.call(request, &mut response, params).await  {
            response = self.run_fallback(sc).await;
        };

        return response;
    }

    pub(crate) async fn run_fallback(&self, status_code: StatusCode) -> Response {

        let key = ServerData::get_fallback_hash_key(&status_code);

        let Some(func) = self.data.get_fallback_func(key) else {
            eprintln!("fallback não encontrado para \'{}\', enviando apenas cabeçalho", status_code.to_string());
            return Response::new().status(status_code).build()
        };

        let mut response = Response::new();
        func.call(status_code, &mut response).await;
        return response;
    }
}



