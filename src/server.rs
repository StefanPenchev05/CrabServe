use std::net::{ IpAddr, Ipv4Addr, SocketAddr };

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

pub struct ServerConfig{
    pub addr: SocketAddr
}

impl ServerConfig {
    pub fn new(ip: [u8; 4], port: u16) -> Self {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::from(ip)), port);
        ServerConfig { addr }
    }

    pub fn set_ip(&mut self, ip: [u8; 4]) {
        self.addr.set_ip(IpAddr::V4(Ipv4Addr::from(ip)));
    }

    pub fn set_port(&mut self, port: u16) {
        self.addr.set_port(port);
    }
}

pub struct CrabServer {
    config: ServerConfig,
}

pub trait Server {
    fn new(config: ServerConfig) -> Self;
    async fn run(&self) -> Result<(), Box<dyn std::error::Error>>;
}

impl Server for CrabServer {
    fn new(config: ServerConfig) -> Self {
        CrabServer { config }
    }

    async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(&self.config.addr).await?;
        println!("Server is listening on http://{}", self.config.addr);

        loop{
            let (socket, _) = listener.accept().await?;
            tokio::spawn(async move {
                handle_connection(socket).await;
            });
        }
    }
}

async fn handle_connection(mut socket:TcpStream) {
    let mut buffer = [0; 1024];

    match socket.read(&mut buffer).await {
        Ok(_) => {
            let response = "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, World!";
            socket.write_all(response.as_bytes()).await.unwrap()
        },
        Err(e) => println!("failed to read from socket; err = {:?}", e)
    }
}