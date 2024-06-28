use std::error::Error;

use CrabServe::server::{ CrabServer, Server };
use CrabServe::database::mongodb::MongoDB;
use CrabServe::db::Database;

#[tokio::main(worker_threads = 3)]
async fn main() -> Result<(), Box<dyn Error>> {
    let server = CrabServer::new([127, 0, 0, 1], 8080);
        match
            server.run(
                Some(
                    Box::pin(async move {
                        let db = MongoDB::new(
                            String::from("mongodb://localhost:27017/"),
                            String::from("Actix_Example")
                        );
                        if let Ok((_, status)) = db.connect().await{
                            println!("{}", status)
                        }
                    })
                ),
                |addr| { println!("Server is listening on http://{}", addr) },
                None
            ).await
        {
            Ok(_) => (),
            Err(err) => panic!("Failed to run server: {:?}", err),
        }
    
    Ok(())
}
