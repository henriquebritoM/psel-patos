
use strum::{EnumIter, IntoEnumIterator};

use crate::ParseErr;

/// As variações mais usuais de status code <br>
/// contém alguns dos principais porque eu não quero <br>
/// passar 1 hora escrevendo variantes <br>
/// fora que isso já me serve muito bem 
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
    fn get_text(&self) -> &'static str {    
        use StatusCode::*;
        return match self {
            //  1xx
            Continue => "Continue",
            SwitchingProtocols => "Switching Protocols",
            Processing => "Processing",
            EarlyHints => "EarlyHints",

            // 2xx
            OK => "OK",
            Created => "Created",
            NoContent => "No Content",
            ResetContent => "Reset Content",
            PartialContent => "Partial Content",
            MultiStatus => "Multi-Status",
            AlreadyReported => "Already Reported",
            ImUsed => "IM Used",

            // 3xx
            MultipleChoices => "Multiple Choices",
            MovedPermanently => "Moved Permanently",
            Found => "Found",
            SeeOther => "See Other",
            NotModified => "Not Modified",
            TemporaryRedirect => "Temporary Redirect",
            PermanentRedirect => "Permanent Redirect",

            //  4xx
            BadRequest => "Bad Request",
            Unauthorized => "Unauthorized",
            PaymentRequired => "Payment Required",
            Forbidden => "Forbidden",
            NotFound => "Not Found",
            MethodNotAllowed => "Method Not Allowed",
            NotAcceptable => "Not Acceptable",
            ProxyAuthenticationRequired => "Proxy Authentication Required",
            RequestTimeout => "Request Timeout",
            Conflict => "Conflict",
            Gone => "Gone",
            LengthRequired => "Length Required",
            PreconditionFailed => "Precondition Failed",
            UriTooLong => "URI Too Long",
            UnsupportedMediaType => "Unsupported Media Type",
            RangeNotSatisfiable => "Range Not Satisfiable",
            ExpectationFailed => "Expectation Failed",
            ImATeapot => "I'm a teapot",
            MisdirectedRequest => "Misdirected Request",
            UnprocessableContent => "Unprocessable Content",
            Locked => "Locked",
            FailedDependency => "Failed Dependency",
            TooEarly => "Too Early",
            UpgradeRequired => "Upgrade Required",
            PreconditionRequired => "Precondition Required",
            TooManyRequests => "Too Many Requests",
            RequestHeaderFieldsTooLarge => "Request Header Fields Too Large",
            UnavailableForLegalReasons => "Unavailable For Legal Reasons",

            //  5xx
            InternalServerError => "Internal Server Error",
            NotImplemented => "Not Implemented",
            BadGateway => "Bad Gateway",
            ServiceUnavailable => "Service Unavailable",
            GatewayTimeout => "Gateway Timeout",
            HttpVersionNotSupported => "HTTP Version Not Supported",
            VariantAlsoNegotiates => "Variant Also Negotiates",
            InsufficientStorage => "Insufficient Storage",
            LoopDetected => "Loop Detected",
            NotExtended => "Not Extended",
            NetworkAuthenticationRequired => "Network Authentication Required",
        }
    }

    pub fn is_err(&self) -> bool {
        return self.get_code() >= 300;
    }

}

impl ToString for StatusCode {
    fn to_string(&self) -> String {
        return format!("{} {}", self.get_code(), self.get_text());
    }
}
