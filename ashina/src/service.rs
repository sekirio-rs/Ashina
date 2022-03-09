//! service handler
use crate::net::http::*;

pub fn simple_handler(req: &HttpRequest) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    log::info!("handle request: {:?}", req);

    let resp = format!(
        "HTTP/1.1 200 OK\r\nServer: {}\r\n\r\n{}",
        "Ashina", "Goobye, Sekiro."
    );

    Ok(HttpResponse::from_bytes(resp.as_bytes())?)
}
