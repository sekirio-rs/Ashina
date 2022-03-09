//! service handler
use crate::net::http;

pub fn simple_handler(
    req: &http::Request,
) -> Result<http::Response, Box<dyn std::error::Error + Send>> {
    log::info!("handle request: {:?}", req);

    let resp = format!(
        "HTTP/1.1 200 OK\r\nServer: {}\r\n\r\n{}",
        "Ashina", "Goobye, Sekiro."
    );

    Ok(http::Response::Raw(resp))
}
