use std::{collections::HashMap, net::{TcpListener, TcpStream, ToSocketAddrs}};

use http_parser::{Method, Request, Response, StatusCode};
use matchit::Router;
use tcp_wrapper::{read_request, write_stream};

use crate::Client;


//  String ok -> tranforma no body da response
/// Retorne sua response completa <br>
/// ou apenas o código para Client/Server error
type Func = fn(&mut Server, &mut Request) -> Result<Response, StatusCode>; //  Um ponteiro para um função estática no código. Parte read only da memória. text. estática.
type FallbackFunc = fn() -> Response;

pub struct Server {
    listener: TcpListener,
    router: Router<Func>,                //  Uma URL router
    fallbacks: HashMap<u16, FallbackFunc>,  
    apis: HashMap<String, Client>,
    keep_alive: bool
}

impl Server {
    pub fn init<A: ToSocketAddrs>(addr: A) -> Server {

        //  tcp listener para o servidor
        let listener = TcpListener::bind(addr).expect("Não foi possível conectar-se a porta");
        return Server { listener, router: Router::new(), fallbacks: HashMap::new(), apis: HashMap::new(), keep_alive: false };
    }

    pub fn get_stream(&mut self) -> &mut TcpListener {
        return &mut self.listener;
    }

    pub fn add_fun(&mut self, method: Method, path: &str, f: Func) {
        let key = Server::get_router_path(method, path);
        self.router.insert(key, f).expect("path passado não é válido. Confira https://docs.rs/matchit/latest/matchit/all.html");
    }

    pub fn add_fallback_fun(&mut self, status_code: StatusCode, f: FallbackFunc) {
        let key = status_code.get_code();
        self.fallbacks.insert(key, f);
    }

    pub fn add_api(&mut self, name: &str, api: Client) {
        self.apis.insert(name.to_string(), api);
    }

    pub fn api_send(&mut self, api_name: &str, req: &mut Request) -> Response {
        let client = self.apis.get_mut(api_name).expect("\'{}\' não é o nome de uma api");

        client.send(req);
        let response = client.receive();
        return response;
    }

    pub fn run(mut self) {

        //  Aceita diversos clients
        loop {
            println!("\n\n------------------------------\n\n");
            println!("waiting");
            let stream = self.listener.accept();
            //  Ignora conexões falhas
            match stream {
                Err(e) => println!("Erro ao conectar-se a stream: {e}"),
                Ok((s, _)) => self.handle_connection(s),
            };
        }
    }

    fn handle_connection(&mut self, mut client_stream: TcpStream) {
        // client_stream.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
        // client_stream.set_write_timeout(Some(Duration::from_secs(5))).unwrap();
        loop {
            let Some(request) = read_request(&mut client_stream) else {return;};    //  Outro lado se desconectou, aborto

            let response = match request {
                Err(_) => {
                    eprintln!("Request fora de formato, rodando fallback BadRequest");
                    self.run_fallback(StatusCode::BadRequest)
                }
                Ok(mut r) => self.get_response(&mut r)
            };

            let Ok(_) = write_stream(&mut client_stream, &response.as_bytes()) else {return;};    //  Outro lado se desconectou, aborto

            if !self.keep_alive {println!("Fechando stream");break;}
        }
    }

    pub fn keep_alive(&mut self, keep_alive: bool) {
        self.keep_alive = keep_alive;
        // return req.headers.get_header_value("Connection") == Some("keep-alive".to_owned());
    }

    fn get_response(&mut self, request: &mut Request) -> Response {

        let key = Server::get_router_path(request.method, &request.path);
        let Some(func) = self.get_func(&key) else {
            eprintln!("Função não encontrada, rodando fallback");
            return self.run_fallback(StatusCode::NotFound);
        };

        return match func(self, request) {
            Ok(r) => r,
            Err(sc) => {
                eprintln!("Função retornou um status code {}, rodando fallback", sc.to_string());
                self.run_fallback(sc)
            },
        }
    }

    fn run_fallback(&self, status_code: StatusCode) -> Response {

        let key = Server::get_fallback_hash_key(&status_code);
        let func = match self.get_fallback_func(key) {
            Some(f) => f,
            None => {
                eprintln!("fallback não encontrado para \'{}\', enviando apenas cabeçalho", status_code.to_string());
                return Response::new().status(status_code).build()
            },
        };

        return func();
    }

    fn get_func(&self, key: &str) -> Option<&Func> {
        return match self.router.at(key) {
            Ok(matched) => Some(matched.value),
            Err(_) => None,
        }
    }

    fn get_fallback_func(&self, key: u16) -> Option<&FallbackFunc> {
        self.fallbacks.get(&key)
    }

    //  PORQUE um ' ' (whitespace) separando os campos?
    //  durante um parsing de Request/Response um ' ' no meio do path
    //  resultaria em um erro. Portanto não é possível acessar keys arbitrárias com o path.
    /// converte o método path passado para router path
    fn get_router_path(method: Method, path: &str) -> String {
        return method.to_string() + " " + path;
    }

    fn get_fallback_hash_key(status_code: &StatusCode) -> u16 {
        return status_code.get_code();
    }
}