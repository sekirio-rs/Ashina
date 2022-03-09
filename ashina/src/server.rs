use crate::net::{http::*, ITcpListener, ITcpStream};
use crate::runtime::Runtime;
use std::error::Error as StdError;

const BUFFER_SIZE: usize = 1024;

pub struct HttpServerBuilder<'ashina> {
    ip: Option<&'ashina str>,
    port: Option<u16>,
}

impl<'ashina> HttpServerBuilder<'ashina> {
    pub fn new() -> Self {
        Self {
            ip: None,
            port: None,
        }
    }
    pub fn ip(&mut self, ip: &'ashina str) -> &mut Self {
        self.ip = Some(ip);
        self
    }
    pub fn port(&mut self, port: u16) -> &mut Self {
        self.port = Some(port);
        self
    }
}

pub struct Server<T, R>
where
    T: ITcpListener,
    R: Runtime,
{
    listener: T,
    _marker: std::marker::PhantomData<R>,
}

impl<T: ITcpListener + 'static, R: Runtime> Server<T, R> {
    pub async fn serve(
        self,
        service_fn: impl Fn(&HttpRequest) -> Result<HttpResponse, Box<dyn StdError>>
            + Send
            + Copy
            + 'static,
    ) -> Result<(), Box<dyn StdError>> {
        loop {
            // Asynchronously wait for an inbound socket.
            let (mut stream, socket_addr) = self.listener.accept().await?;
            log::debug!("accept, socket addr: {:?}", socket_addr);

            R::spawn(async move {
                let mut buf = vec![0; BUFFER_SIZE];

                let _n = stream
                    .read(&mut buf)
                    .await
                    .expect("failed to read data from socket");

                let request = HttpRequest::from_bytes(&buf).unwrap_or_default();
                let response = service_fn(&request).unwrap_or_default();

                if let Err(e) = stream.write(response.as_bytes()).await {
                    log::error!("send response error: {:?}", e);
                }
            });
        }
    }
}
