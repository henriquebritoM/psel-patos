use crate::{Header, Protocol, Response, StatusCode};

pub struct ResponseBuilder {
    protocol: Option<Protocol>,
    status: Option<StatusCode>,
    headers: Header,
    body: Vec<u8>,
}

impl ResponseBuilder {

    pub fn new() -> ResponseBuilder {
        return ResponseBuilder {
            protocol: None,
            status: None,
            headers: Header::new(),
            body: Vec::new(),
        };
    }

    pub fn protocol(mut self, p: Protocol) -> ResponseBuilder {
        self.protocol = Some(p);
        return self;
    }

    pub fn status(mut self, sc: StatusCode) -> ResponseBuilder {
        self.status = Some(sc);
        return self;
    }

    pub fn add_header<T: ToString, U: ToString>(mut self, field: T, value: U) -> ResponseBuilder {
        self.headers.add_header(field, value);
        return self;
    } 

    pub fn body<T: Into<Vec<u8>>>(mut self, body: T) -> ResponseBuilder {
        self.body = body.into();
        let len = self.body.len();
        self.add_header("Content-Length", len)
    }

    pub fn build(self) -> Response {
        
        Response { 
            protocol: self.protocol.unwrap_or(Protocol::Http11),
            status: self.status.unwrap_or(StatusCode::NotFound),
            headers: self.headers,
            body: self.body
        }
    }
}
