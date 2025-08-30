use thiserror::Error;

/// Os tipos de erros que podem acontecer durante o parsing
/// de uma &str para HttpRequest
/// todos irrecuperáveis
#[derive(Debug, Error)]
pub enum ParseErr {
    #[error("Request fora do padrão: \"{}\"", .0)]
    BadFormat(String),
    #[error("Método não reconhecido: \"{}\"", .0)]
    BadMethod(String),
    #[error("Path não reconhecido \"{}\"", .0)]
    BadPath(String),
    #[error("Protocolo invalido: \"{}\"", .0)]
    BadProtocol(String)
}

