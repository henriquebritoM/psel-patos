
#[cfg(test)] 
mod testes {

    use http_parser::*;

    #[test]
    fn test_to_default_to_string() {
        let response = Response::new();

        let expected = b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n";

        assert_eq!(response.as_bytes(), expected);
    }

    #[test]
    fn test_with_body() {
        let response = Response::new().status(StatusCode::OK).body("12345".as_bytes()).build();

        let expected = "HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\n12345";

        assert_eq!(String::from_utf8_lossy(&response.as_bytes()), expected);
    }

    #[test]
    fn from_bytes_test() {

        let bytes = 
        [
        "HTTP/1.1 200 OK\r\n",
        "Content-Type: application/json\r\n",
        "Content-Length: 48\r\n",
        "Connection: close\r\n",
        "\r\n",
        r#"["documento.pdf", "imagem.jpg", "dados.json"]"#,
        ].join("");

        let parsed = Response::try_from(bytes.as_bytes()).unwrap();

        assert_eq!(String::from_utf8_lossy(&parsed.as_bytes()), bytes);
    }

    #[test]
    fn to_bytes_test() {

        let response = Response::new()
                                    .status(StatusCode::NotFound)
                                    .add_header("Content-Length", 5)
                                    .body("12345").build();

        let expected = 
        [
        "HTTP/1.1 404 Not Found\r\n",
        "Content-Length: 5\r\n",
        "\r\n",
        "12345",
        ].join("");

        assert_eq!(String::from_utf8_lossy(&response.as_bytes()).to_string(), expected);
    }
}
