use std::{collections::HashMap, net::{TcpListener, TcpStream, ToSocketAddrs}};

use http_parser::{Method, Request, Response, StatusCode};
use tcp_wrapper::{read_request, write_stream};

use crate::Client;


//  String ok -> tranforma no body da response
/// Retorne sua response completa <br>
/// ou apenas o código para Client/Server error
type Func = fn(&mut Server, &mut Request) -> Result<Response, StatusCode>; //  Um ponteiro para um função estática no código. Parte read only da memória. text. estática.
type FallbackFunc = fn() -> Response;

pub struct Server {
    listener: TcpListener,
    endpoints: HashMap<String, Func>,
    fallbacks: HashMap<u16, FallbackFunc>,
    apis: HashMap<String, Client>,
    keep_alive: bool
}

impl Server {
    pub fn init<A: ToSocketAddrs>(addr: A) -> Server {

        //  tcp listener para o servidor
        let listener = TcpListener::bind(addr).expect("Não foi possível conectar-se a porta");
        return Server { listener, endpoints: HashMap::new(), fallbacks: HashMap::new(), apis: HashMap::new(), keep_alive: false };
    }

    pub fn get_stream(&mut self) -> &mut TcpListener {
        return &mut self.listener;
    }

    pub fn add_fun(&mut self, method: Method, path: &str, f: Func) {
        let key = Server::get_hash_key(method, path);
        self.endpoints.insert(key, f);
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
                Err(_) => println!("Erro ao receber stream"),
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
                Err(_) => self.run_fallback(StatusCode::BadRequest),
                Ok(mut r) => {
                    let res = self.get_response(&mut r);
                    // keep_alive = self.keep_alive(&r);
                    res
                }
            };

            println!("response body len = {}", response.body.len());

            let Ok(_) = write_stream(&mut client_stream, &response.as_bytes()) else {continue;};
            
            println!("Manter tcp ? {}", self.keep_alive);
            println!("Response ok? {}", !response.status.is_err());
            println!("status {}", response.status.to_string());
            if !self.keep_alive {println!("breaking");break;}
        }
    }

    pub fn keep_alive(&mut self, keep_alive: bool) {
        self.keep_alive = keep_alive;
        // return req.headers.get_header_value("Connection") == Some("keep-alive".to_owned());
    }

    fn get_response(&mut self, request: &mut Request) -> Response {

        let key = Server::get_hash_key(request.method, &request.path);
        let Some(func) = self.get_func(&key) else {
            return self.run_fallback(StatusCode::NotFound);
        };

        return match func(self, request) {
            Ok(r) => r,
            Err(sc) => self.run_fallback(sc),
        }
    }

    fn run_fallback(&self, status_code: StatusCode) -> Response {

        let key = Server::get_fallback_hash_key(&status_code);
        let func = match self.get_fallback_func(key) {
            Some(f) => f,
            None => return Response::new().status(status_code).build(),
        };

        return func();
    }

    fn get_func(&self, key: &str) -> Option<&Func> {
        return self.endpoints.get(key)
    }

    fn get_fallback_func(&self, key: u16) -> Option<&FallbackFunc> {
        self.fallbacks.get(&key)
    }

    fn get_hash_key(method: Method, path: &str) -> String {
        return method.to_string() + "-" + path;
    }

    fn get_fallback_hash_key(status_code: &StatusCode) -> u16 {
        return status_code.get_code();
    }
}