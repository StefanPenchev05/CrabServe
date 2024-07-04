use std::str::FromStr;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
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

pub enum ContentType {
    ApplicationJson,
    TextHtml,
    TextPlain,
    ApplicationXml,
    ApplicationXhtmlXml,
    ApplicationJavascript,
    ApplicationFormUrlencoded,
    MultipartFormData,
    ImagePng,
    ImageJpeg,
    ImageGif,
    Custom(String),
}

pub trait ParseFromString: Sized {
    type Err;
    fn from_str(s: &str) -> Result<Self, Self::Err>;
}

impl FromStr for ContentType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "application/json" => Ok(ContentType::ApplicationJson),
            "text/html" => Ok(ContentType::TextHtml),
            "text/plain" => Ok(ContentType::TextPlain),
            "application/xml" => Ok(ContentType::ApplicationXml),
            "application/xhtml+xml" => Ok(ContentType::ApplicationXhtmlXml),
            "application/javascript" => Ok(ContentType::ApplicationJavascript),
            "application/x-www-form-urlencoded" => Ok(ContentType::ApplicationFormUrlencoded),
            "multipart/form-data" => Ok(ContentType::MultipartFormData),
            "image/png" => Ok(ContentType::ImagePng),
            "image/jpeg" => Ok(ContentType::ImageJpeg),
            "image/gif" => Ok(ContentType::ImageGif),
            other => Ok(ContentType::Custom(other.to_string())),
        }
    }
}