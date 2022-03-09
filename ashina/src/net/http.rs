//! HTTP protocol
//! should be moved under sekirio-rs

pub type HttpHeader = std::collections::HashMap<String, String>;

#[derive(Default, Debug)]
pub struct HttpRequest {
    pub req_line: HttpRequestLine,
    pub header: HttpHeader,
    pub body: Vec<u8>,
}

#[derive(Default, Debug)]
pub struct HttpRequestLine {
    pub func: HttpRequestFunction,
    pub url: String,
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
    pub fn from_bytes(_bytes: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        todo!()
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
    pub fn from_bytes(_bytes: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        todo!()
    }
}
