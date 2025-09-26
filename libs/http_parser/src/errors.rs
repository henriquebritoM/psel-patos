use thiserror::Error;

/// Os tipos de erros que podem acontecer durante o parsing
/// de um &[u8] para Request e Response
/// todos irrecuperáveis
#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum ParseErr {
    #[error("Request fora do padrão: \"{}\"", .0)]
    BadFormat(String),
    #[error("Método não reconhecido: \"{}\"", .0)]
    BadMethod(String),
    #[error("Path não reconhecido \"{}\"", .0)]
    BadPath(String),
    #[error("Protocolo invalido: \"{}\"", .0)]
    BadProtocol(String),
    #[error("StatusCode invalido: \"{}\"", .0)]
    BadStatusCode(String),
}
