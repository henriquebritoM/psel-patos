use std::{fs::read, path::Path};

use http_parser::{Request, Response, StatusCode};
use smol_server::Server;

pub fn get_index(_server: &mut Server, _req: &mut Request) -> Result<Response, StatusCode> {
    let path: &Path = Path::new(r#"C:\Users\henri\prog_things\rust projects\patos_psel\pages\main_page\index.html"#);
    let body = read(path).ok().ok_or(StatusCode::NotFound)?;
    
    return Ok(Response::new().status(StatusCode::OK).body(body).build());

}

pub fn get_css(_server: &mut Server, _req: &mut Request) -> Result<Response, StatusCode> {
    let path: &Path = Path::new(r#"C:\Users\henri\prog_things\rust projects\patos_psel\pages\main_page\assets\css\style.css"#);

    let body = read(path).ok().ok_or(StatusCode::NotFound)?;
    Ok(Response::new().body(body).status(StatusCode::OK).build())
}

pub fn get_image(_server: &mut Server, _req: &mut Request) -> Result<Response, StatusCode> {
    let path: &Path = Path::new(r#"C:\Users\henri\prog_things\rust projects\patos_psel\pages\main_page\assets\image.jpg"#);

    let body_bytes = read(path).ok().ok_or(StatusCode::NotFound)?;

    Ok(Response::new().add_header("Content-Type", "image/jpg")
                      .add_header("Content-Length", &body_bytes.len())
                      .body(body_bytes)
                      .status(StatusCode::OK)
                      .build())
}

pub fn get_script(_server: &mut Server, _req: &mut Request) -> Result<Response, StatusCode> {
    let path: &Path = Path::new(r#"C:\Users\henri\prog_things\rust projects\patos_psel\pages\main_page\assets\js\script.js"#);

    let body = read(path).ok().ok_or(StatusCode::NotFound)?;

    Ok(Response::new().body(body).status(StatusCode::OK).build())
}
