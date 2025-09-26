use crate::{Header, Method, Protocol, Request};

pub struct RequestBuilder {
    method: Option<Method>,
    path: String,
    protocol: Option<Protocol>,
    headers: Header,
    body: Vec<u8>,
}

impl RequestBuilder {

    pub fn new() -> RequestBuilder {
        return RequestBuilder {
            method: None,
            path: String::new(),
            protocol: None,
            headers: Header::new(),
            body: Vec::new(),
        };
    }

    pub fn method(mut self, m: Method) -> RequestBuilder {
        self.method = Some(m);
        return self;
    }

    pub fn path(mut self, p: &str) -> RequestBuilder {
        self.path.push_str(p);
        return self;
    }

    pub fn protocol(mut self, p: Protocol) -> RequestBuilder {
        self.protocol = Some(p);
        return self;
    }

    pub fn add_header<T: ToString, U: ToString>(mut self, field: T, value: U) -> RequestBuilder {
        self.headers.add_header(field, value);
        return self;
    } 

    pub fn body<T: Into<Vec<u8>>>(mut self, body: T) -> RequestBuilder {
        self.body = body.into();
        let len = self.body.len();
        self.add_header("Content-Length", len)
    }

    pub fn build(self) -> Request {
        
        Request { 
            method: self.method.unwrap_or(Method::GET),
            path: if self.path.is_empty() {"/".to_string()} else {self.path},
            protocol: self.protocol.unwrap_or(Protocol::Http11),
            headers: self.headers,
            body: self.body
        }
    }
}
