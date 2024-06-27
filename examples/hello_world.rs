use std::error::Error;

use CrabServe::server::{ CrabServer, Server };

#[tokio::main(worker_threads = 3)]
async fn main() -> Result<(), Box<dyn Error>> {
    let server = CrabServer::new([127,0,0,1], 8080);
    server.run(|addr|  println!("Server listening on http://{}", addr) ).await?;

    Ok(())
}
