use async_trait::async_trait;
use std::{ collections::HashMap, str::FromStr };
use serde::{ Deserialize, Serialize };
use serde_json::Value;
use super::http_types::ContentType;
use thiserror::Error;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

#[derive(Error, Debug)]
pub enum RequestError {
    #[error("Failed to parse request line")]
    RequestLineParseError,
    #[error("Method not found in request")]
    MethodNotFoundError,
    #[error("Path not found in request")]
    PathNotFoundError,
    #[error("Headers/Body delimiter not found")]
    HeadersBodyDelimiterNotFoundError,
    #[error("Malformed header line: '{0}'")] MalformedHeaderError(String),
    #[error("Failed to parse Content-Type: '{0}'")] ContentTypeParseError(String),
    #[error("Unsupported content type")]
    UnsupportedContentTypeError,
    #[error("Failed to process JSON body: {0}")] JsonBodyProcessingError(serde_json::Error),
}

#[async_trait]
pub trait HttpRequest {
    fn new(method: &str, path: &str) -> Self;

    fn set_headers(&mut self, headers: HashMap<String, String>);
    fn get_headers(&self) -> &HashMap<String, String>;
    fn set_body(&mut self, body: Vec<u8>);
    fn get_body(&self) -> &[u8];

    fn add_header(&mut self, key: &str, value: &str);

    fn add_body(&mut self, body: Vec<u8>) {
        self.set_body(body);
    }

    fn method(&self) -> &str;
    fn path(&self) -> &str;

    async fn parse(raw_req: &str) -> Result<Self, RequestError> where Self: Sized;
    async fn process_json_body(json_body: &str) -> Result<Value, serde_json::Error>;
    fn set_body_and_content_length(headers: &mut HashMap<String, String>, body: &[u8]);
}


#[async_trait]
impl HttpRequest for Request {
    fn new(method: &str, path: &str) -> Self {
        Self {
            method: method.to_string(),
            path: path.to_string(),
            headers: HashMap::new(),
            body: Vec::new(),
        }
    }

    async fn process_json_body(json_body: &str) -> Result<Value, serde_json::Error> {
        let result = async {
            serde_json::from_str(json_body)
        }.await;

        result
    }

    fn set_body_and_content_length(headers: &mut HashMap<String, String>, body: &[u8]) {
        headers.insert("Content-Length".to_string(), body.len().to_string());
    }

    async fn parse(raw_req: &str) -> Result<Self, RequestError> where Self: Sized{
        let result = (async {
            let mut lines = raw_req.lines();

            let request_line = lines.next().ok_or(RequestError::RequestLineParseError)?;
            let mut parts = request_line.split_whitespace();
            let method = parts.next().ok_or(RequestError::MethodNotFoundError)?;
            let path = parts.next().ok_or(RequestError::PathNotFoundError)?;

            let (headers, body) = raw_req
                .split_once("\r\n\r\n")
                .ok_or(RequestError::HeadersBodyDelimiterNotFoundError)?;

            let mut headers_map: HashMap<String, String> = headers
                .lines()
                .skip(1)
                .filter_map(|line| {
                    let mut parts = line.splitn(2, ':');
                    match (parts.next(), parts.next()) {
                        (Some(key), Some(value)) =>
                            Some((key.trim().to_string(), value.trim().to_string())),
                        _ => None,
                    }
                })
                .collect();

            let binding = String::from("");
            let content_type_str = headers_map.get("Content-Type").unwrap_or(&binding);
            let content_type = ContentType::from_str(&content_type_str).unwrap();
            let body_bytes = match content_type {
                ContentType::ApplicationJson => {
                    let json_value: serde_json::Value = serde_json
                        ::from_str(body)
                        .map_err(RequestError::JsonBodyProcessingError)?;
                    serde_json::to_vec(&json_value).map_err(RequestError::JsonBodyProcessingError)?
                }

                _ => body.as_bytes().to_vec(),
            };

            Self::set_body_and_content_length(&mut headers_map, &body_bytes);

            Ok(Self {
                method: method.to_string(),
                path: path.to_string(),
                headers: headers_map,
                body: body_bytes,
            })
        }).await;

        result
    }

    fn add_header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }

    fn set_headers(&mut self, headers: HashMap<String, String>) {
        self.headers = headers;
    }

    fn get_headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    fn set_body(&mut self, body: Vec<u8>) {
        self.body = body;
    }

    fn get_body(&self) -> &[u8] {
        &self.body
    }

    fn method(&self) -> &str {
        &self.method
    }

    fn path(&self) -> &str {
        &self.path
    }
}
