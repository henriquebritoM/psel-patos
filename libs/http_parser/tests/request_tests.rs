
#[cfg(test)] 
mod testes {

    use http_parser::*;

    #[test]
    fn test_to_default_to_string() {
        let request = Request::new().build();

        let expected= b"GET / HTTP/1.1\r\n\r\n";

        assert_eq!(&request.as_bytes(), expected);
    }
}



