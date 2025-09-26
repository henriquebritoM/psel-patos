
#[cfg(test)] 
mod testes {

    use http_parser::*;

    #[test]
    fn test_to_default_to_string() {
        let response = Response::new().build();

        let expected = b"HTTP/1.1 404 Not Found\r\n\r\n";

        assert_eq!(response.as_bytes(), expected);
    }

    #[test]
    fn test_with_body() {
        let response = Response::new().status(StatusCode::OK).body("TESTANDO".as_bytes()).build();

        let expected = b"HTTP/1.1 200 OK\r\n\r\nTESTANDO\r\n";

        assert_eq!(&response.as_bytes(), expected);
    }
}



