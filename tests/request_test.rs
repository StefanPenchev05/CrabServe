#[cfg(test)]
mod tests {
    use CrabServe::http_core::request::{HttpRequest, Request};

    #[test]
    fn test_request_setter_and_getters() {
        let mut request = Request::new("GET", "/person/greeting");

        // Add a header
        request.add_header("Content-Type", "application/json");
        request.add_header("Authorization", "Bearer token123");

        // Set the body
        let body_content = "{\"name\":\"John Doe\"}";
        request.set_body(body_content.as_bytes().to_vec());

        // Retrieve and assert headers
        let headers = request.get_headers();
        assert_eq!(headers.get("Content-Type").unwrap(), "application/json");
        assert_eq!(headers.get("Authorization").unwrap(), "Bearer token123");

        // Retrieve and assert body
        let body = request.get_body();
        assert_eq!(body, body_content.as_bytes());
    }

    #[test]
    fn test_parse_request() {
        let raw_request =
            "GET /hello HTTP/1.1\r\nContent-Type: application/json\r\nAuthorization: Bearer token123\r\n\r\n{\"name\":\"John Doe\"}";
        let request = Request::parse(raw_request).unwrap();

        // Assert method and path
        assert_eq!(request.method(), "GET");
        assert_eq!(request.path(), "/hello");

        // Assert headers
        let headers = request.get_headers();
        assert_eq!(headers.get("Content-Type").unwrap(), "application/json");
        assert_eq!(headers.get("Authorization").unwrap(), "Bearer token123");
        assert!(headers.contains_key("Content-Length")); // Assert Content-Length is set

        // Assert body
        let expected_body = "{\"name\":\"John Doe\"}".as_bytes();
        assert_eq!(request.get_body(), expected_body);
    }

    #[test]
    fn test_parse_request_more_larger_request() {
        let raw_request =
            "POST /api/users HTTP/1.1\r\nContent-Type: application/json\r\nAuthorization: Bearer token123\r\nCustom-Header: CustomValue\r\nContent-Length: 47\r\n\r\n{\"name\":\"John Doe\", \"email\":\"john@example.com\"}";
        let request = Request::parse(raw_request).unwrap();

        // Assert method and path
        assert_eq!(request.method(), "POST");
        assert_eq!(request.path(), "/api/users");

        // Assert headers
        let headers = request.get_headers();
        assert_eq!(headers.get("Content-Type").unwrap(), "application/json");
        assert_eq!(headers.get("Authorization").unwrap(), "Bearer token123");
        assert_eq!(headers.get("Custom-Header").unwrap(), "CustomValue");
        assert!(headers.contains_key("Content-Length"));

        // Assert body
        let expected_body = "{\"name\":\"John Doe\",\"email\":\"john@example.com\"}".as_bytes().to_vec();
        assert_eq!(request.get_body(), expected_body);
    }
}
