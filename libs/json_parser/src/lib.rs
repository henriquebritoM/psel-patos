use std::{fs::File, io::{Read, Write}, net::SocketAddr, path::Path};

use serde_json;

pub fn save(path: &Path, data: SocketAddr) -> Result<(), ()>{

    let mut file: File = std::fs::OpenOptions::new().write(true)
                                                    .create(true)
                                                    .open(path)
                                                    .or(Err(()))?;

    let json_t = serde_json::to_string_pretty(&data).or(Err(()))?;

    file.write_all(json_t.as_bytes()).or(Err(()))?;

    Ok(())
}

pub fn retrieve(path: &Path) -> Result<SocketAddr, ()> {

    let mut file = std::fs::File::open(path).or(Err(()))?;
    let mut json_str: String = String::new(); 

    file.read_to_string(&mut json_str).or(Err(()))?;

    let data: SocketAddr = serde_json::from_str(&json_str).or(Err(()))?;

    return Ok(data);
}
