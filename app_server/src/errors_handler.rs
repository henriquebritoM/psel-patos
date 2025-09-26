use std::{fs::read, path::Path};

use http_parser::{Response, StatusCode};

pub fn not_found() -> Response {
    let path: &Path = Path::new(r#"C:\Users\henri\prog_things\rust projects\patos_psel\pages\errors\404.html"#);

    match read(path) {
        Ok(body) => Response::new().status(StatusCode::NotFound).body(body).build(),
        Err(_) => Response::new().build(),
    }
}

pub fn not_allowed() -> Response {
    let path: &Path = Path::new(r#"C:\Users\henri\prog_things\rust projects\patos_psel\pages\errors\405.html"#);

    match read(path) {
        Ok(body) => Response::new().status(StatusCode::MethodNotAllowed).body(body).build(),
        Err(_) => Response::new().build(),
    }   
}

pub fn bad_request() -> Response {
    let path: &Path = Path::new(r#"C:\Users\henri\prog_things\rust projects\patos_psel\pages\errors\400.html"#);

    match read(path) {
        Ok(body) => Response::new().status(StatusCode::BadRequest).body(body).build(),
        Err(_) => Response::new().build(),
    }   
}

pub fn server_error() -> Response {
    let path: &Path = Path::new(r#"C:\Users\henri\prog_things\rust projects\patos_psel\pages\errors\500.html"#);

    match read(path) {
        Ok(body) => Response::new().status(StatusCode::InternalServerError).body(body).build(),
        Err(_) => Response::new().build(),
    }   
}

pub fn http_not_supported() -> Response {
    let path: &Path = Path::new(r#"C:\Users\henri\prog_things\rust projects\patos_psel\pages\errors\505.html"#);

    match read(path) {
        Ok(body) => Response::new().status(StatusCode::HttpVersionNotSupported).body(body).build(),
        Err(_) => Response::new().build(),
    }   
}

