//! HTTP protocol
//!
//! currently unused
use std::error::Error as StdError;

pub type HttpHeader = std::collections::HashMap<String, String>;

// ----- request -----

#[derive(Debug)]
pub enum Request {
    Raw(String),
    Parsed(HttpRequest),
}

impl Request {
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Request::Raw(s) => s.as_bytes(),
            Request::Parsed(r) => r.as_bytes(),
        }
    }
}

/// HTTP request
#[derive(Default, Debug)]
pub struct HttpRequest {
    /// request line
    pub req_line: HttpRequestLine,
    /// request header
    pub header: HttpHeader,
    /// request body
    pub body: Vec<u8>,
}

/// HTTP request line
#[derive(Default, Debug)]
pub struct HttpRequestLine {
    /// request function
    pub func: HttpRequestFunction,
    /// request url
    pub url: String,
    /// http version
    pub version: (u16, u16),
}

#[derive(Debug)]
pub enum HttpRequestFunction {
    GET,
    POST,
    PUT,
    HEAD,
    DELETE,
    TRACE,
    CONNECT,
    OPTIONS,
}

impl std::default::Default for HttpRequestFunction {
    fn default() -> Self {
        Self::GET
    }
}

impl HttpRequest {
    pub fn from_bytes(_bytes: &[u8]) -> Result<Self, Box<dyn StdError>> {
        todo!()
    }

    pub fn as_bytes(&self) -> &[u8] {
        todo!()
    }
}

// ----- response -----

pub enum Response {
    Raw(String),
    Parsed(HttpResponse),
}

impl Response {
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Response::Raw(s) => s.as_bytes(),
            Response::Parsed(r) => r.as_bytes(),
        }
    }
}

pub struct HttpResponse {
    pub status_line: HttpResponseStatusLine,
    pub headers: HttpHeader,
    pub body: Vec<u8>,
}

pub struct HttpResponseStatusLine {
    pub verson: (u16, u16),
    pub code: u16,
    pub msg: String,
}

impl std::default::Default for HttpResponse {
    fn default() -> Self {
        todo!()
    }
}

impl HttpResponse {
    pub fn as_bytes(&self) -> &[u8] {
        todo!()
    }
    pub fn from_bytes(_bytes: &[u8]) -> Result<Self, Box<dyn StdError>> {
        todo!()
    }
}
