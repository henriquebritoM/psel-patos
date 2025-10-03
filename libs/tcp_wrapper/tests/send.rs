
#[cfg(test)] 
mod testes {

    use tokio::net::{TcpListener, TcpStream};

    use http_parser::*;
    use tcp_wrapper::*;

    #[tokio::test]
    async fn test_to_default_to_string() {
        let listener = TcpListener::bind("localhost:9090").await.unwrap();
        let mut sender = TcpStream::connect("localhost:9090").await.unwrap();

        let req_enviada = Request::new()
                            .method(http_parser::Method::DELETE)
                            .path("/index.html")
                            .protocol(Protocol::Http2)
                            .add_header("Content-Length", "3")
                            .body("123");   

        let (mut receiver, _): (TcpStream, std::net::SocketAddr) = listener.accept().await.unwrap();

        write_stream(&mut sender, &req_enviada.as_bytes()).await.unwrap();

        let req_recebida = read_request(&mut receiver).await;

        assert_eq!(Some(Ok(req_enviada)), req_recebida);
    }
}



