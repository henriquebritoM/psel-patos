
#[cfg(test)] 
mod testes {

    use http_parser::*;

    #[test]
    fn test_to_default_to_string() {
        let request = Request::new();

        let expected= b"GET / HTTP/1.1\r\nContent-Length: 0\r\n\r\n";

        assert_eq!(&request.as_bytes(), expected);
    }

        #[test]
    fn from_bytes_test() {

        let bytes = 
        [
        "GET /files HTTP/1.1\r\n",
        "Host: example.com\r\n",
        "User-Agent: MeuCliente/1.0\r\n",
        "Accept: application/json, text/plain, */*\r\n",
        "Connection: close\r\n",
        "\r\n",
        ].join("");

        let parsed = Request::try_from(bytes.as_bytes()).unwrap();

        assert_eq!(String::from_utf8_lossy(&parsed.as_bytes()), bytes);
    }

    #[test]
    fn to_bytes_test() {

        let request = Request::new()
                                    .method(Method::GET)
                                    .path("/files")
                                    .add_header("Content-Length", 5)
                                    .body("12345").build();

        let expected = 
        [
        "GET /files HTTP/1.1\r\n",
        "Content-Length: 5\r\n",
        "\r\n",
        "12345",
        ].join("");

        assert_eq!(String::from_utf8_lossy(&request.as_bytes()).to_string(), expected);
    }
}



