use std::net::{ IpAddr, Ipv4Addr, SocketAddr };

use tokio::io::{ AsyncReadExt, AsyncWriteExt };
use tokio::net::{ TcpListener, TcpStream };

#[derive(Debug)]
pub struct CrabServer {
    pub addr: SocketAddr,
}

pub trait Server {
    fn new(ip: [u8; 4], port: u16) -> Self;
    async fn run<F>(
        &self,
        database_connection: Option<Box<dyn Fn() + Send>>,
        clouser: F
    ) -> Result<(), Box<dyn std::error::Error>>
        where F: FnMut(&SocketAddr) + Send;
}

impl Server for CrabServer {
    fn new(ip: [u8; 4], port: u16) -> Self {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::from(ip)), port);
        CrabServer { addr }
    }

    async fn run<F>(
        &self,
        database_connection: Option<Box<dyn Fn() + Send>>,
        mut clouser: F
    ) -> Result<(), Box<dyn std::error::Error>>
        where F: FnMut(&SocketAddr) + Send
    {
        let listener = TcpListener::bind(&self.addr).await?;

        match database_connection {
            Some(db_connection) => db_connection(),
            None => println!("No DataBase Initialized")
        }
        clouser(&self.addr);

        loop {
            let (socket, _) = listener.accept().await?;
            tokio::spawn(async move {
                handle_connection(socket).await;
            });
        }
    }
}

async fn handle_connection(mut socket: TcpStream) {
    let mut buffer = [0; 1024];

    match socket.read(&mut buffer).await {
        Ok(_) => {
            let response = "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, World!";
            socket.write_all(response.as_bytes()).await.unwrap()
        }
        Err(e) => println!("failed to read from socket; err = {:?}", e),
    }
}
