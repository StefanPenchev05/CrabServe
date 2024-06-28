#[cfg(test)]
mod tests {
    use std::{ net::SocketAddr, sync::Arc, time::Duration };
    use tokio::{ io::{ AsyncReadExt, AsyncWriteExt }, net::TcpStream, sync::{ oneshot, Mutex } };
    use CrabServe::{ database::mongodb::MongoDB, db::Database, server::{ CrabServer, Server } };

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
                .run(
                    None,
                    |addr| { println!("Server is listening on http://{}", addr) },
                    None
                ).await
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

    #[tokio::test]
    async fn test_mongodb_connection_with_server_running() {
        let (tx, rx) = oneshot::channel();
        let server = CrabServer::new([127, 0, 0, 1], 8080);
        let db_connection_status = Arc::new(Mutex::new(false));
        let db_connection_status_clone = db_connection_status.clone();
        let server_handler = tokio::spawn(async move {
            match
                server.run(
                    Some(
                        Box::pin(async move {
                            let db = MongoDB::new(
                                String::from("mongodb://localhost:27017/"),
                                String::from("Actix_Example")
                            );
                            let result = tokio::task
                                ::spawn(async move {
                                    db.connect().await.map_err(|e| e.to_string())
                                }).await
                                .expect("Task failed");

                            match result {
                                Ok(_) => {
                                    let mut status = db_connection_status_clone.lock().await;
                                    *status = true;
                                    println!("Connection Established");
                                }
                                Err(e) => println!("Failed to connect to MongoDB: {:?}", e),
                            }

                            tx.send(());
                        })
                    ),
                    |addr| { println!("Server is listening on http://{}", addr) },
                    Some(rx)
                ).await
            {
                Ok(_) => (),
                Err(err) => panic!("Failed to run server: {:?}", err),
            }
        });


        let _ = server_handler.await.expect("Server handler failed to run");

        let connected = db_connection_status.lock().await;
        println!("{}", *connected);
        assert!(*connected, "Connection between the server and database failed!");
    }
}
