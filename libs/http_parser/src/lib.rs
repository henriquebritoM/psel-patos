/*  Minha própria biblioteca, feita exclusivamente para esse psel
 *  Contém structs relacionadas a HTTP requests e responses
 *  e métodos de parsing sofisticados, que não crasham, mas
 *  tratam dos erros, insano
 *
 *  Vi os outros psels e ponderei se isso não era overkill, mas já era tarde demais pra voltar atrás
 *
*/

pub mod errors;
pub mod request;
pub mod response;
pub mod parts;
pub use errors::ParseErr;
pub use request::request::Request;
pub use response::response::Response;
pub use parts::{header::Header, method::Method, protocol::Protocol, status_code::StatusCode};
