use crate::http_core::{ response::Response, http_types::HttpMethods };

pub type RouterHandler = fn() -> Response;

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct RouteKey {
    pub method: HttpMethods,
    pub path: String,
}

#[derive(Clone)]
pub struct Route {
    pub handler: RouterHandler,
    pub middleware: Vec<fn() -> ()>,
}