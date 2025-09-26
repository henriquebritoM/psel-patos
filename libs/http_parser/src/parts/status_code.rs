
use strum::{EnumIter, IntoEnumIterator};

use crate::ParseErr;


/// As variações mais usuais de status code <br>
/// contém alguns dos principais porque eu não quero <br>
/// passar 1 hora escrevendo variantes <br>
/// fora que isso já me serve muito bem 
#[derive(Debug, PartialEq, Eq, Clone, EnumIter)]
pub enum StatusCode {
    OK,
    BadRequest,
    NotFound,
    MethodNotAllowed,
    ImATeapot,
    InternalServerError,
    HttpVersionNotSupported,
    Custom{code: u16, message: String}
}

impl StatusCode {

    pub fn from_str(s: &str, message: Option<&str>) -> Result<StatusCode, ParseErr> {
        let code: u16 = s.parse().ok().ok_or(ParseErr::BadStatusCode(s.to_string()))?;
        if code == 0 {return Err(ParseErr::BadStatusCode(s.to_string()));}

        for status_code in StatusCode::iter() {
            if status_code.get_code() == code {
                return Ok(status_code);
            }
        }

        let Some(message) = message else {return Err(ParseErr::BadStatusCode("\"no message\"".to_string()));};

        return Ok(StatusCode::Custom { code: code, message: message.to_string() });
    }

    /// retorna o código do status code
    pub fn get_code(&self) -> u16 {
        use StatusCode::*;
        return match self {
            OK => 200,
            BadRequest => 400,
            NotFound => 404,
            MethodNotAllowed => 405,
            ImATeapot => 418,
            InternalServerError => 500,
            HttpVersionNotSupported => 505,
            Custom { code, ..} => *code
        }
    }

    /// retorna a mensagem descritiva do status code
    fn get_text(&self) -> String {    
        use StatusCode::*;
        return match self {
            OK => "OK",
            BadRequest => "Bad Request",
            NotFound => "Not Found",
            MethodNotAllowed => "Method Not Allowed",
            ImATeapot => "I'm a teapot",
            InternalServerError => "Internal Server Error",
            HttpVersionNotSupported => "HTTP Version Not Supported",
            Custom {message, ..} => message,
        }.into();
    }

    pub fn is_err(&self) -> bool {
        use StatusCode::*;
        return match self {
            BadRequest => true,
            NotFound => true,
            MethodNotAllowed => true,
            ImATeapot => true,
            InternalServerError => true,
            HttpVersionNotSupported => true,
            _ => false,
        };
    }

}

impl ToString for StatusCode {
    fn to_string(&self) -> String {
        return format!("{} {}", self.get_code(), self.get_text());
    }
}
