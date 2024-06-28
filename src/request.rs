use std::collections::HashMap;

pub enum HttpMethods {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
    PATCH,
    CONNECT,
    TRACE,
}

impl HttpMethods {
    pub fn to_string(&self) -> &str {
        match self {
            HttpMethods::GET => "GET",
            HttpMethods::POST => "POST",
            HttpMethods::PUT => "PUT",
            HttpMethods::DELETE => "DELETE",
            HttpMethods::HEAD => "HEAD",
            HttpMethods::OPTIONS => "OPTIONS",
            HttpMethods::PATCH => "PATCH",
            HttpMethods::CONNECT => "CONNECT",
            HttpMethods::TRACE => "TRACE",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

pub trait HttpRequest {
    // Constructor to create a new Request instance
    fn new(method: &str, path: &str) -> Self;

    // Define setters and getters for fields that the trait methods will modify
    fn set_headers(&mut self, headers: HashMap<String, String>);
    fn get_headers(&self) -> &HashMap<String, String>;
    fn set_body(&mut self, body: Vec<u8>);
    fn get_body(&self) -> &[u8];

    // Default method implementations
    fn add_header(&mut self, key: &str, value: &str) {
        let mut headers = self.get_headers().clone();
        headers.insert(key.to_string(), value.to_string());
        self.set_headers(headers);
    }

    fn add_body(&mut self, body: Vec<u8>) {
        self.set_body(body);
    }

    // Methods without default implementations
    fn method(&self) -> &str;
    fn path(&self) -> &str;

    // Function to parse a raw HTTP request into a Request instance
    fn parse(raw_req: &str) -> Self;
}

impl HttpRequest for Request {
    fn new(method: &str, path: &str) -> Self {
        Self {
            method: method.to_string(),
            path: path.to_string(),
            headers: HashMap::new(),
            body: Vec::new(),
        }
    }

    fn parse(raw_req: &str) -> Self {
        let mut lines = raw_req.lines();

        let requst_line = lines.next().unwrap_or_default();
        let mut parts = requst_line.split_whitespace();
        let method = parts.next().unwrap_or_default();
        let path = parts.next().unwrap_or_default();

        let (headers, body) = raw_req.split_once("\r\n\r\n").unwrap_or_default();
        let headers_map: HashMap<String, String> = headers
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

        let body_bytes = body.as_bytes().to_vec();

        Self {
            method: method.to_string(),
            path: path.to_string(),
            headers: headers_map,
            body: body_bytes,
        }
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
