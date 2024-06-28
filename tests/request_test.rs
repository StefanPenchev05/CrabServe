#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use CrabServe::request::{ HttpMethods, HttpRequest, Request };

    #[test]
    fn test_request_setter_and_getters() {
        let mut request = Request::new(HttpMethods::GET.to_string(), "/person/greeting");

        // Add a header
        request.add_header("Content-Type", "application/json");
        request.add_header("Authorization", "Bearer token123");

        // Set the body
        let body_content = "{\"name\":\"John Doe\"}";
        request.set_body(body_content.to_string().into_bytes());

        // Retrieve and assert headers
        let headers = request.get_headers();
        assert_eq!(headers.get("Content-Type").unwrap(), "application/json");
        assert_eq!(headers.get("Authorization").unwrap(), "Bearer token123");

        // Retrieve and assert body
        let body = request.get_body();
        assert_eq!(body, body_content.to_string().into_bytes());
    }

    #[test]
    fn test_parse_request() {
        let raw_request =
            "GET /hello HTTP/1.1\r\nContent-Type: application/json\r\nAuthorization: Bearer token123\r\n\r\n{\"name\":\"John Doe\"}";
        let request = Request::parse(raw_request);

        // Assert method and path
        assert_eq!(request.method, "GET");
        assert_eq!(request.path, "/hello");

        // Assert headers
        let mut expected_headers = HashMap::new();
        expected_headers.insert("Content-Type".to_string(), "application/json".to_string());
        expected_headers.insert("Authorization".to_string(), "Bearer token123".to_string());
        assert_eq!(request.headers, expected_headers);

        // Assert body
        let expected_body = "{\"name\":\"John Doe\"}".as_bytes().to_vec();
        assert_eq!(request.body, expected_body);
    }

    #[test]
    fn test_parse_request_more_larger_request() {
        let raw_request =
            "POST /api/users HTTP/1.1\r\nContent-Type: application/json\r\nAuthorization: Bearer token123\r\nCustom-Header: CustomValue\r\nContent-Length: 47\r\n\r\n{\"name\":\"John Doe\", \"email\":\"john@example.com\"}";

        let request = Request::parse(raw_request);

        // Assert method and path
        assert_eq!(request.method, "POST");
        assert_eq!(request.path, "/api/users");

        println!("{:#?}", request.get_headers());

        // Assert headers
        let mut expected_headers = HashMap::new();
        expected_headers.insert("Content-Type".to_string(), "application/json".to_string());
        expected_headers.insert("Authorization".to_string(), "Bearer token123".to_string());
        expected_headers.insert("Custom-Header".to_string(), "CustomValue".to_string());
        expected_headers.insert("Content-Length".to_string(), "47".to_string());
        assert_eq!(request.headers, expected_headers);

        // Assert body
        let expected_body = r#"{"name":"John Doe", "email":"john@example.com"}"#
            .as_bytes()
            .to_vec();
        assert_eq!(request.body, expected_body);
    }
}
