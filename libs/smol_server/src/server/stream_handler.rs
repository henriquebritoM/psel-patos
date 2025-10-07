use http_parser::{Request, Response, StatusCode};
use tcp_wrapper::{read_request, write_stream};
use tokio::{net::TcpStream};

use crate::server::server_data::ServerData;

/// Agrupa os dados necessários para responder Requests
pub(crate) struct ConectionHandler {
    data: &'static ServerData,
    stream: TcpStream
}

impl ConectionHandler {
    /// Entrypoint para criar novas tasks
    pub(crate) async fn handle(data: &'static ServerData, stream: TcpStream) {
        let mut handler = ConectionHandler { data, stream };
        handler.handle_connection().await;
    }

    /// Recebe e responde as requests enviadas
    /// uma stream só é fechado quando uma request ou response 
    /// possui o Header "Connection: Close"
    pub(crate) async fn handle_connection(&mut self) {
        let mut response: Response;
        let mut keep_alive: bool = true;

        while keep_alive {
            
            let Some(request) = read_request(&mut self.stream).await else {return;};    //  Outro lado se desconectou, aborto
            response = match request {
                Ok(r) => {
                    keep_alive = !r.closing();
                    self.get_response(r).await
                },
                Err(_) => {
                    eprintln!("Request fora de formato, rodando fallback BadRequest");
                    self.run_fallback(StatusCode::BadRequest).await
                },
            };

            keep_alive = keep_alive && !response.closing();

            let Ok(_) = write_stream(&mut self.stream, &response.as_bytes()).await else {return;};    //  Outro lado se desconectou, aborto
        }
    }
    
    /// Processa a Request e gera uma response apropriada
    pub(crate) async fn get_response(&mut self, request: Request) -> Response {

        let key = ServerData::get_router_path(request.method, &request.path);
        let Some((func, params)) = self.data.get_func(&key) else {
            eprintln!("Função não encontrada, rodando fallback");
            return self.run_fallback(StatusCode::NotFound).await;
        };

        let response = Response::new();

        let response = match func.call(request, response, params).await {
            Ok(r) => r,
            Err(sc) =>  self.run_fallback(sc).await,
        };

        return response;
    }

    /// Executa a função fallback, se não houver nenhuma, gera uma Response
    /// default para o StatusCode
    pub(crate) async fn run_fallback(&self, status_code: StatusCode) -> Response {

        let key = ServerData::get_fallback_hash_key(&status_code);

        let Some(func) = self.data.get_fallback_func(key) else {
            return Response::default(status_code);
        };

        let mut response = func.call().await;
        response.close();   

        return response;
    }
}



