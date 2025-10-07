use std::collections::HashMap;

//  Considerei criar um enum para cada variante de header, 
//  mas não tenho certeza sobre o quanto isso seria útil
//  fica aqui uma approach mais naive, mas que cumpre o objetivo
/// Struct que permite acesso e manipulação de headers http
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct  Header {
    headers: HashMap<String, String>,
}

impl Header {

    /// Cria uma instância de header
    pub fn new() -> Header {

        let hash: HashMap<String, String> = HashMap::new();

        return Header { headers: hash };
    }

    //  Adiciona um header à struct
    pub fn add_header<T: ToString, U: ToString>(&mut self, field: T, value: U) {
        self.headers.insert(field.to_string(),value.to_string());
    }

    /// Retorna o valor de um header
    pub fn get_header(&self, field: &str) -> Option<String> {
        return match self.headers.get(field).cloned() {
            Some(s) => Some(s.trim().to_string()),
            None => None,
        }
        
    }

    /// Remove o header da struct
    pub fn remove_header(&mut self, field: &str) {
        self.headers.remove(field);
    }

    pub fn is_empty(&self) -> bool {
        return self.headers.is_empty();
    }
}

// Implementação do trait From<&str> para Header
impl From<&str> for Header {
    fn from(value: &str) -> Self {
        let mut header = Header::new();

        //  Separa 's' em linhas, depois
        //  separa cada linha no ':', 
        //  resultado: ("Content-Length", "123")
        let parts: Vec<(&str, &str)> =  value.split("\r\n").map(|value| value.split_once(": ").unwrap_or(("", "") ) ).collect();

        for part in parts {
            if part.0.is_empty() {continue;}    //  Ignoramos as que não tem nada no primeiro argumento
            
            header.add_header(part.0, part.1);
        }

        return header;
    }
}

// Implementação do trait ToString para header
impl ToString for Header {
    fn to_string(&self) -> String {
        //  itera sobre os elementod de self.headers, concatenando cada par no estilo padrão para ser enviado
        //  resultado: "Content-Length: 123"
        return self.headers.iter().map(|(s1, s2)| format!("{}: {}\r\n", s1, s2).to_string()).collect()
    }
}