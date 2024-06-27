use crate::db::Database;
use async_trait::async_trait;
use serde::{ Deserialize, Serialize };
use mongodb::{options::{Credential, TlsOptions, ClientOptions} ,Client };

pub struct MongoDB {
    pub connection_string: String,
    pub database_name: String,
    pub auth_username: Option<String>,
    pub auth_password: Option<String>,
    pub auth_source: Option<String>,
    pub use_tls: bool,
    pub tls_certificate_path: Option<String>,
    pub max_pool_size: Option<u32>,
    pub min_pool_size: Option<u32>,
    pub socket_timeout_ms: Option<u64>,
    pub server_selection_timeout_ms: Option<u64>,
    pub heartbeat_frequency_ms: Option<u64>,
    pub read_preference: Option<String>,
    pub write_concern: Option<String>,
    pub read_concern: Option<String>,
    pub retry_writes: bool,
    pub retry_reads: bool,
    pub app_name: Option<String>,
    pub compression: Option<String>,
}

#[async_trait]
impl Database for MongoDB {

    type ConnectionType = Client;

    fn new(connection_string: String, database_name: String) -> Self {
        Self {
            connection_string,
            database_name,
            auth_username: None,
            auth_password: None,
            auth_source: None,
            use_tls: false,
            tls_certificate_path: None,
            max_pool_size: None,
            min_pool_size: None,
            socket_timeout_ms: None,
            server_selection_timeout_ms: None,
            heartbeat_frequency_ms: None,
            read_preference: None,
            write_concern: None,
            read_concern: None,
            retry_writes: true,
            retry_reads: true,
            app_name: None,
            compression: None,
        }
    }

    async fn connect(&self) -> Result<Client, Box<dyn std::error::Error>> {
        let mut client_options = ClientOptions::parse(self.connection_string.to_string()).await?;

        if let (Some(username), Some(password)) = (&self.auth_username, &self.auth_password){
            let credentials = Credential::builder()
                .username(self.auth_username.clone())
                .password(self.auth_password.clone())
                .source(self.auth_source.clone())
                .build();

            client_options.credential = Some(credentials);
        }

        if self.use_tls{
            use std::path::PathBuf;
            
            let tls_options = TlsOptions::builder()
                .ca_file_path(self.tls_certificate_path.clone().map(PathBuf::from))
                .build();
        
            client_options.tls = Some(tls_options.into());
        }

        let client = Client::with_options(client_options)?;
        Ok(client)
    }

    async fn query(&self, query: String) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
