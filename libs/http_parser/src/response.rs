use crate::parsers::protocol_parser::Protocol;

/// Struct para montagem modular de uma response HTTP
pub struct Response {
    pub protocol: Protocol,
    pub status: StatusCode,
    pub headers: Vec<String>,
    pub body: String
}

impl Response {

    /// Retorna uma instância de Response
    pub fn new(protocol: Protocol, status: StatusCode, headers: Vec<String>, body: String) -> Response {

        return Response {
            protocol,
            status,
            headers,
            body
        };
    }
}

/// Implementação do trait ToString para Response
/// Transforma um Response em String pronta para ser enviada
/// ao client
impl ToString for Response {
    fn to_string(&self) -> String {
        let mut temp_str = String::new();
        
        temp_str.push_str(&self.protocol.to_string());
        temp_str.push_str(&self.status.to_string());
        temp_str.push_str("\r\n");

        if !self.headers.is_empty() {
            for s in self.headers.iter() {
                temp_str.push_str(s);
                temp_str.push_str("\r\n");
            }
        }

        temp_str.push_str("\r\n");

        if !self.body.is_empty() {
            temp_str.push_str(&self.body);
        }

        temp_str.push_str("\r\n");

        return temp_str;
    }
}
    

/// Struct statuc code
/// utilidade: Dar match nas constantes ou só 
/// colocar na response para facilitar 
/// ainda falta uma revisão nessa parte 
/// todo()
pub struct StatusCode {
    code: u16,
    message: &'static str
}

impl StatusCode {

    //  Alguns códigos que irei utilizar para o PSEL, tem muitos outros mas não
    //  quero ter que implementar todos, ainda mais dessa forma
    pub const OK: StatusCode = StatusCode::new_unchecked(200, "OK");

    pub const BAD_REQUEST: StatusCode = StatusCode::new_unchecked(400, "Bad Request");
    pub const NOT_FOUND: StatusCode = StatusCode::new_unchecked(404, "Not Found");
    pub const IM_A_TEAPOT: StatusCode = StatusCode::new_unchecked(418, "I'm a teapot");

    pub const SERVER_ERROR: StatusCode = StatusCode::new_unchecked(500, "Internal Server Error");
    pub const HTTP_VERSION_NOT_SUPPORTED: StatusCode = StatusCode::new_unchecked(505, "HTTP Version Not Supported");

    const fn new_unchecked(code: u16, message:&'static str) -> StatusCode {
        if code == 0 {panic!()};

        return StatusCode {
            code,
            message
        };
    }
}

impl ToString for StatusCode {
    fn to_string(&self) -> String {
        return format!("{} {}", self.code, self.message);
    }
}
