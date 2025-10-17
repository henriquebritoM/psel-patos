use std::{fs::{read, write}, path::{Path, PathBuf}};

use http_parser::{Request, Response, StatusCode};
use smol_server::Params;

/// Retorna o item especificado pelo path
pub async fn get_item(req: Request, mut res: Response, _params: Params) -> Result<Response, StatusCode> {
    //  formata o caminho
    let path_buff: PathBuf = PathBuf::from(r"./".to_string() + &req.path);
    let path: &Path = Path::new(&path_buff);

    let body = read(path).ok().ok_or(StatusCode::NotFound)?;
    
    res.status(StatusCode::OK).body(body);

    let content_type: Option<String> = match req.headers.get_header("Content-Type") {
        Some(ct) => Some(ct),
        None => get_content_type(&req).map(|s| s.to_string()),
    };

    if let Some(ct) = content_type {
        res.add_header("Content-Type", ct);
    }

    return Ok(res);
}

/// Cria um novo arquivo
pub async fn post_item(req: Request, mut res: Response, _params: Params) -> Result<Response, StatusCode> {

    let path_buff: PathBuf = PathBuf::from(r"./".to_string() + &req.path);
    let path: &Path = Path::new(&path_buff);

    if path.exists() {return Err(StatusCode::Conflict);}
    let Ok(_) = write(path, &req.body) else {return Err(StatusCode::BadRequest)};

    res.status(StatusCode::Created);
    res.body(req.path.clone());
    return Ok(res);
}

/// Lista todos os arquivos disponíveis
pub async fn list_files(_req: Request, mut res: Response, _params: Params) -> Result<Response, StatusCode> {

    let mut file_names: Vec<String> = Vec::new();

    if let Ok(paths) = std::fs::read_dir("./files") {
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

    return Ok(res);
}

/// Retorna o content-type de uma response (se houver)
fn get_content_type(req: &Request) -> Option<&'static str>{
    let content_type: Option<&'static str> = req.path.rsplit_once(".")
        .map(|(_, ext)| match ext {
            "png"  => "image/png",
            "jpg" | "jpeg" => "image/jpeg",
            "gif"  => "image/gif",
            "svg"  => "image/svg+xml",
            "css" | "txt"  => "text/css; charset=utf-8",
            "js"   => "application/javascript; charset=utf-8",
            "html" => "text/html; charset=utf-8",
            "json" => "application/json",
            _      => "application/octet-stream",
    }); 

    return content_type;
}
