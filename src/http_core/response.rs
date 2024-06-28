use std::{ collections::HashMap, fmt::format };
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Response {
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Response {
    pub fn new(status_code: u16) -> Self {
        Self {
            status_code,
            headers: HashMap::new(),
            body: Vec::new(),
        }
    }

    pub fn set_json_body<T: Serialize>(mut self, data: &T) -> serde_json::Result<Self> {
        let serialized = serde_json::to_string(data)?;
        self.headers.insert("Content-Type".to_string(), "application/json".to_string());
        self.body = serialized.into();
        Ok(self)
    }

    pub fn format(&self) -> Result<Vec<u8>, std::string::FromUtf8Error> {
        let status_line = format!(
            "HTTP/1.1 {} {}\r\n",
            self.status_code,
            self.get_status_message()
        );

        let headers = self.headers
            .iter()
            .map(|(key, value)| format!("{}: {}\r\n", key, value))
            .collect::<String>();

        let body = String::from_utf8(self.body.clone())?;

        Ok(format!("{}{}\r\n{}", status_line, headers, body).into_bytes())
    }

    fn get_status_message(&self) -> &str {
        match self.status_code {
            200 => "OK",
            201 => "Created",
            400 => "Bad Request",
            401 => "Unauthorized",
            403 => "Forbidden",
            404 => "Not Found",
            500 => "Internal Server Error",
            _ => "Unknown",
        }
    }

    pub fn add_header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn add_body(mut self, body: Vec<u8>) -> Self {
        self.body = body;
        self
    }

    pub fn status_code(&self) -> u16 {
        self.status_code
    }

    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    pub fn body(&self) -> &[u8] {
        &self.body
    }
}
