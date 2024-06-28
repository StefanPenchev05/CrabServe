use std::net::{ IpAddr, Ipv4Addr, SocketAddr };
use std::pin::Pin;
use std::future::Future;
use tokio::io::{ AsyncReadExt, AsyncWriteExt };
use tokio::net::{ TcpListener, TcpStream };
use tokio::sync::oneshot;
use log::{ info, error };

#[derive(Debug)]
pub struct CrabServer {
    pub addr: SocketAddr,
}

#[allow(async_fn_in_trait)]
pub trait Server {
    fn new(ip: [u8; 4], port: u16) -> Self;
    async fn run(
        &self,
        database_connection: Option<Pin<Box<dyn Future<Output = ()> + Send + 'static>>>,
        on_listen: impl FnMut(&SocketAddr) + Send + 'static,
        shutdown_signal: Option<oneshot::Receiver<()>>
    ) -> Result<(), Box<dyn std::error::Error>>;
}

impl Server for CrabServer {
    fn new(ip: [u8; 4], port: u16) -> Self {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::from(ip)), port);
        CrabServer { addr }
    }

    async fn run(
        &self,
        database_connection: Option<Pin<Box<dyn Future<Output = ()> + Send + 'static>>>,
        mut on_listen: impl FnMut(&SocketAddr) + Send + 'static,
        shutdown_signal: Option<oneshot::Receiver<()>>
    ) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(&self.addr).await?;

        if let Some(db_connection) = database_connection {
            db_connection.await;
        } else {
            info!("No DataBase Initialized");
        }

        on_listen(&self.addr);
        info!("Server listening on {}", self.addr);

        match shutdown_signal {
            Some(shutdown) => {
                tokio::select! {
                    res = accept_connections(listener) => {
                        if let Err(e) = res {
                            error!("Error accepting connections: {}", e);
                        }
                    },
                    _ = shutdown => {
                        info!("Shutdown signal received, stopping server.");
                    }
                }
            }
            None => {
                accept_connections(listener).await?;
            }
        }

        Ok(())
    }
}

async fn accept_connections(listener: TcpListener) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket).await {
                error!("Failed to handle connection: {}", e);
            }
        });
    }
}

async fn handle_connection(mut socket: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = [0; 1024];

    match socket.read(&mut buffer).await {
        Ok(_) => {
            let response = "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, World!";
            socket.write_all(response.as_bytes()).await?;
        }
        Err(e) => {
            error!("Failed to read from socket; err = {:?}", e);
        }
    }

    Ok(())
}
