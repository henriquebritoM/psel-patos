
use strum::{EnumIter, IntoEnumIterator};

use crate::ParseErr;

/// Enum com as principais variantes de 
/// status code
#[derive(Debug, PartialEq, Eq, Clone, Copy, EnumIter)]
#[repr(u16)]
pub enum StatusCode {
    //  1xx Informational
    Continue = 100,
    SwitchingProtocols = 101,
    Processing = 102,
    EarlyHints = 103,

    //  2xx Success
    OK = 200, 
    Created = 201,
    NoContent = 204,
    ResetContent = 205,
    PartialContent = 206,
    MultiStatus = 207,
    AlreadyReported = 208,
    ImUsed = 226,

    // 3xx  Redirection
    MultipleChoices = 300,
    MovedPermanently = 301,
    Found = 302,
    SeeOther = 303,
    NotModified = 304,
    TemporaryRedirect = 307,
    PermanentRedirect = 308,

    // 4xx Client Error
    BadRequest = 400,
    Unauthorized = 401,
    PaymentRequired = 402,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    NotAcceptable = 406,
    ProxyAuthenticationRequired = 407,
    RequestTimeout = 408,
    Conflict = 409,
    Gone = 410,
    LengthRequired = 411,
    PreconditionFailed = 412,
    UriTooLong = 414,
    UnsupportedMediaType = 415,
    RangeNotSatisfiable = 416,
    ExpectationFailed = 417,
    ImATeapot = 418,
    MisdirectedRequest = 421,
    UnprocessableContent = 422,
    Locked = 423,
    FailedDependency = 424,
    TooEarly = 425,
    UpgradeRequired = 426,
    PreconditionRequired = 428,
    TooManyRequests = 429,
    RequestHeaderFieldsTooLarge = 431,
    UnavailableForLegalReasons = 451,

    // 5xx Server Error
    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
    GatewayTimeout = 504,
    HttpVersionNotSupported = 505,
    VariantAlsoNegotiates = 506,
    InsufficientStorage = 507,
    LoopDetected = 508,
    NotExtended = 510,
    NetworkAuthenticationRequired = 511
    // Custom{code: u16, message: String, is_err: bool} = 0,    //  Fora do escopo do projeto
}

impl StatusCode {

    /// Cria um status code a partir de uma str
    pub fn from_str(s: &str) -> Result<StatusCode, ParseErr> {
        let code: u16 = s.parse().unwrap_or(0);
        if code == 0 {return Err(ParseErr::BadStatusCode(s.to_string()));}

        for status in StatusCode::iter() {
            if status.get_code() == code {return Ok(status);}
        }
        return Err(ParseErr::BadStatusCode(s.to_string()));

    }

    /// retorna o código do status code
    pub fn get_code(&self) -> u16 {
        *self as u16
    }

    /// retorna a mensagem descritiva do status code
    pub fn get_text(&self) -> &'static str {    
        use StatusCode as SC;
        return match self {
            //  1xx
            SC::Continue => "Continue",
            SC::SwitchingProtocols => "Switching Protocols",
            SC::Processing => "Processing",
            SC::EarlyHints => "EarlyHints",

            // 2xx
            SC::OK => "OK",
            SC::Created => "Created",
            SC::NoContent => "No Content",
            SC::ResetContent => "Reset Content",
            SC::PartialContent => "Partial Content",
            SC::MultiStatus => "Multi-Status",
            SC::AlreadyReported => "Already Reported",
            SC::ImUsed => "IM Used",

            // 3xx
            SC::MultipleChoices => "Multiple Choices",
            SC::MovedPermanently => "Moved Permanently",
            SC::Found => "Found",
            SC::SeeOther => "See Other",
            SC::NotModified => "Not Modified",
            SC::TemporaryRedirect => "Temporary Redirect",
            SC::PermanentRedirect => "Permanent Redirect",

            //  4xx
            SC::BadRequest => "Bad Request",
            SC::Unauthorized => "Unauthorized",
            SC::PaymentRequired => "Payment Required",
            SC::Forbidden => "Forbidden",
            SC::NotFound => "Not Found",
            SC::MethodNotAllowed => "Method Not Allowed",
            SC::NotAcceptable => "Not Acceptable",
            SC::ProxyAuthenticationRequired => "Proxy Authentication Required",
            SC::RequestTimeout => "Request Timeout",
            SC::Conflict => "Conflict",
            SC::Gone => "Gone",
            SC::LengthRequired => "Length Required",
            SC::PreconditionFailed => "Precondition Failed",
            SC::UriTooLong => "URI Too Long",
            SC::UnsupportedMediaType => "Unsupported Media Type",
            SC::RangeNotSatisfiable => "Range Not Satisfiable",
            SC::ExpectationFailed => "Expectation Failed",
            SC::ImATeapot => "I'm a teapot",
            SC::MisdirectedRequest => "Misdirected Request",
            SC::UnprocessableContent => "Unprocessable Content",
            SC::Locked => "Locked",
            SC::FailedDependency => "Failed Dependency",
            SC::TooEarly => "Too Early",
            SC::UpgradeRequired => "Upgrade Required",
            SC::PreconditionRequired => "Precondition Required",
            SC::TooManyRequests => "Too Many Requests",
            SC::RequestHeaderFieldsTooLarge => "Request Header Fields Too Large",
            SC::UnavailableForLegalReasons => "Unavailable For Legal Reasons",

            //  5xx
            SC::InternalServerError => "Internal Server Error",
            SC::NotImplemented => "Not Implemented",
            SC::BadGateway => "Bad Gateway",
            SC::ServiceUnavailable => "Service Unavailable",
            SC::GatewayTimeout => "Gateway Timeout",
            SC::HttpVersionNotSupported => "HTTP Version Not Supported",
            SC::VariantAlsoNegotiates => "Variant Also Negotiates",
            SC::InsufficientStorage => "Insufficient Storage",
            SC::LoopDetected => "Loop Detected",
            SC::NotExtended => "Not Extended",
            SC::NetworkAuthenticationRequired => "Network Authentication Required",
        }
    }

    /// Status code maiores ou iguais a 400 são considerados erros
    pub fn is_err(&self) -> bool {
        return self.get_code() >= 400;
    }

}

//  Implementação do trait ToString para StatusCode
impl ToString for StatusCode {
    fn to_string(&self) -> String {
        return format!("{} {}", self.get_code(), self.get_text());
    }
}
