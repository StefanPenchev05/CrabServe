#[cfg(test)]
mod tests {
    use std::net::{ SocketAddr };
    use tokio::{ io::{ AsyncReadExt, AsyncWriteExt }, net::TcpStream };
    use CrabServe::server::{ CrabServer, Server };

    #[tokio::test]
    async fn test_server_addr() {
        let server = CrabServer::new([127, 0, 0, 1], 8080);
        let addr = server.addr;

        let mut closure_called = false;

        let mut clouser = |server_addr: &SocketAddr| {
            assert_eq!(server_addr, &addr);
            closure_called = true;
        };

        clouser(&addr);
        assert!(closure_called, "The closure should have been called with the server's address.");
    }

    #[tokio::test]
    async fn test_server_run() {
        let server = CrabServer::new([127, 0, 0, 1], 8080);
        let server_task = tokio::spawn(async move {
            server
                .run(None, |addr| { println!("Server is listening on http://{}", addr) }).await
                .unwrap()
        });

        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

        let mut stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();

        let request = "GET / HTTP/1.1\r\n\r\n";
        stream.write_all(request.as_bytes()).await.unwrap();

        let mut buffer = [0; 1024];
        let n = stream.read(&mut buffer).await.unwrap();
        let response = String::from_utf8_lossy(&buffer[..n]);

        assert!(response.contains("HTTP/1.1 200 OK"));
        assert!(response.contains("Hello, World!"));

        server_task.abort();
    }
}
