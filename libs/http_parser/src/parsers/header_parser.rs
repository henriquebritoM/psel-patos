
//  Eu iria colocar uma variante de enum para cada tipo de header
//  mas isso seria quase inútil nessa aplicação
//  aqui jáz o resto de código que sobrou dessa efêmera ideia
#[derive(Debug)]
pub struct  Header {}


impl Header {

    /// Retorna um vec de strings, cada string é um header
    pub fn get_vec(s: &str) -> Vec<String> {
        return s.split("\r\n").map(|s: &str| s.to_string()).collect();
    }
}