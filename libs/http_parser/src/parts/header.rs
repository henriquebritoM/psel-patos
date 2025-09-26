use std::collections::HashMap;

//  Eu iria colocar uma variante de enum para cada tipo de header
//  mas isso seria quase inútil nessa aplicação
//  aqui jáz o resto de código que sobrou dessa efêmera ideia
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct  Header {
    headers: HashMap<String, String>,
}

impl Header {

    /// Retorna um vec de strings, cada string é um header
    pub fn new() -> Header {

        let hash: HashMap<String, String> = HashMap::new();

        return Header { headers: hash };
    }

    //  Adiciona um único header
    pub fn add_header<T: ToString, U: ToString>(&mut self, field: T, value: U) {
        self.headers.insert(field.to_string(),value.to_string());
    }

    pub fn get_header_value(&self, field: &str) -> Option<String> {
        return match self.headers.get(field).cloned() {
            Some(s) => Some(s.trim().to_string()),
            None => None,
        }
        
    }

    pub fn remove_header(&mut self, field: &str) {
        self.headers.remove(field);
    }

    pub fn is_empty(&self) -> bool {
        return self.headers.is_empty();
    }
}

impl From<&str> for Header {
    fn from(value: &str) -> Self {
        let mut header = Header::new();

        //  Separa 's' por linhas, depois
        //  separa cada linha por ':', depois
        let parts: Vec<(&str, &str)> =  value.split("\r\n").map(|value| value.split_once(": ").unwrap_or(("", "") ) ).collect();

        for part in parts {
            if part.0.is_empty() {continue;}    //  Ignoramos as que não tem nada no primeiro argumento
            
            header.add_header(part.0, part.1);
        }

        return header;
    }
}

impl ToString for Header {
    fn to_string(&self) -> String {
        return self.headers.iter().map(|(s1, s2)| format!("{}: {}\r\n", s1, s2).to_string()).collect()
    }
}