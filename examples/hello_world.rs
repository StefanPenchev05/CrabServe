use CrabServe::server::{CrabServer, Server, ServerConfig};

#[tokio::main]
async fn main() {

    let config = ServerConfig::new([127, 0,0,1], 3030);
    let server = CrabServer::new(config);
    if let Err(e) = server.run().await {
        eprintln!("Server error: {}", e);
    }
}