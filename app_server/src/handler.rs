use std::{fs::{read, write}, path::{self, Path, PathBuf}};

use http_parser::{Request, Response, StatusCode};
use smol_server::Server;

pub fn get_item(_server: &mut Server, req: &mut Request) -> Result<Response, StatusCode> {
    //  formata o caminho
    let path_buff: PathBuf = PathBuf::from(r"..".to_string() + &req.path);
    let path: &Path = Path::new(&path_buff);
    println!("buscando em {:?}", path_buff);

    let body = read(path).ok().ok_or(StatusCode::NotFound)?;
    let mut response = Response::new().status(StatusCode::OK).body(body).build();

    if let Some(ct) = get_content_type(req) {
        response.headers.add_header("Content-Type", ct);
    }

    return Ok(response);
}

pub fn post_item(_server: &mut Server, req: &mut Request) -> Result<Response, StatusCode> {

    let path_buff: PathBuf = PathBuf::from(r"..".to_string() + &req.path);
    let path: &Path = Path::new(&path_buff);

    if path.exists() {return Err(StatusCode::Conflict);}
    let Ok(_) = write(path, &req.body) else {return Err(StatusCode::BadRequest)};

    return Ok(
        Response::new()
        .status(StatusCode::Created)
        .body(req.path.clone())
        .build()
    );
}

/// Retorna o content-type de uma response (se houver)
fn get_content_type(req: &Request) -> Option<&'static str>{
    let content_type: Option<&'static str> = req.path.rsplit_once(".")
        .map(|(_, ext)| match ext {
            "png"  => "image/png",
            "jpg" | "jpeg" => "image/jpeg",
            "gif"  => "image/gif",
            "svg"  => "image/svg+xml",
            "css"  => "text/css; charset=utf-8",
            "js"   => "application/javascript; charset=utf-8",
            "html" => "text/html; charset=utf-8",
            "json" => "application/json",
            _      => "application/octet-stream",
    }); 

    return content_type;
}
