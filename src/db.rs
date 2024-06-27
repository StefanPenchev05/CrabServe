use async_trait::async_trait;

#[async_trait]
pub trait Database {

    type ConnectionType;
    fn new(connection_string: String, database_name: String) -> Self;
    async fn connect(&self) -> Result<Self::ConnectionType, Box<dyn std::error::Error>>;
    async fn query(&self, query: String) -> Result<(), Box<dyn std::error::Error>>;
}