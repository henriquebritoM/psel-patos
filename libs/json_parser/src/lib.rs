use std::{fs::File, io::{Read, Write}, path::Path};

use serde::{de::DeserializeOwned, Serialize};
use serde_json;

/// Salva o argumento no path passado <br>
/// **Limpa o arquivo de destino**
pub fn save<T: Serialize + DeserializeOwned>(path: &Path, data: T) -> Result<(), ()> {

    let mut file: File = std::fs::OpenOptions::new().write(true)
                                                    .create(true)
                                                    .truncate(true)
                                                    .open(path)
                                                    .or(Err(()))?;

    let json_t = serde_json::to_string_pretty(&data).or(Err(()))?;

    file.write_all(json_t.as_bytes()).or(Err(()))?;

    Ok(())
}

/// Recupera um objeto do path passado
/// **Não limpa o arquivo de destino**
pub fn retrieve<T: Serialize + DeserializeOwned>(path: &Path) -> Result<T, ()> {

    let mut file = std::fs::File::open(path).or(Err(()))?;
    let mut json_str: String = String::new(); 

    file.read_to_string(&mut json_str).or(Err(()))?;

    let data: T = serde_json::from_str(&json_str).or(Err(()))?;

    return Ok(data);
}
