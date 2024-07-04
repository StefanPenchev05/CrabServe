use std::collections::HashMap;

use crate::http_core::response::Response;
use super::route::{ Route, RouteKey, RouterHandler };

struct Router {
    routes: Option<HashMap<RouteKey, Route>>,
    path: String,
    pub error_handlers: Option<HashMap<u16, fn() -> Response>>,
    pub ssl_certificate: Option<String>,
    pub ssl_private_key: Option<String>,
}

impl Router {
    fn new(path: String) -> Self {
        Self {
            routes:None,
            path,
            error_handlers: None,
            ssl_certificate: None,
            ssl_private_key: None,
        }
    }
}
