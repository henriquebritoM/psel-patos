use std::{fs::{read, write}, path::{Path, PathBuf}};

use http_parser::{Request, Response, StatusCode};
use smol_server::Params;

pub fn get_item(req: Request, res: &mut Response, _params: Params) -> Result<(), StatusCode> {
    //  formata o caminho
    let path_buff: PathBuf = PathBuf::from(r"..".to_string() + &req.path);
    let path: &Path = Path::new(&path_buff);
    println!("buscando em {:?}", path_buff);

    let body = read(path).ok().ok_or(StatusCode::NotFound)?;
    
    res.status(StatusCode::OK).body(body);

    if let Some(ct) = req.headers.get_header_value("Content-Type") {
        res.add_header("Content-Type", ct);
    }

    return Ok(());
}

pub fn post_item(req: Request, res: &mut Response, _params: Params) -> Result<(), StatusCode> {

    let path_buff: PathBuf = PathBuf::from(r"..".to_string() + &req.path);
    let path: &Path = Path::new(&path_buff);

    if path.exists() {return Err(StatusCode::Conflict);}
    let Ok(_) = write(path, &req.body) else {return Err(StatusCode::BadRequest)};

    res.status(StatusCode::Created);
    res.body(req.path.clone());
    return Ok(());
}

pub fn list_files(_req: Request, res: &mut Response, _params: Params) -> Result<(), StatusCode> {

    let mut file_names: Vec<String> = Vec::new();

    if let Ok(paths) = std::fs::read_dir("../files") {
        for path in paths {
            if let Ok(f) = path {
                file_names.push(f.file_name().to_string_lossy().to_string());
            };
        }
    }

    let Ok(json) = serde_json::to_string(&file_names) else {return Err(StatusCode::InternalServerError);};

    res.status(StatusCode::OK);
    res.add_header("Content-Type", "application/json");
    res.body(json);

    return Ok(());
}

