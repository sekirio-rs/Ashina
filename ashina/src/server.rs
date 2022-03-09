use crate::net::{http, ITcpListener, ITcpStream};
use crate::runtime::Runtime;
use std::error::Error as StdError;
use std::marker::PhantomData;

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
    pub async fn build<T, R>(&self) -> Result<Server<T, R>, Box<dyn StdError>>
    where
        T: ITcpListener,
        R: Runtime,
    {
        let ip = self.ip.unwrap_or("127.0.0.1");
        let port = self.port.unwrap_or(80);
        let addr = format!("{}:{}", ip, port);
        let listener = T::bind(&addr).await?;

        Ok(Server {
            listener,
            _marker: PhantomData,
        })
    }
}

pub struct Server<T, R>
where
    T: ITcpListener,
    R: Runtime,
{
    listener: T,
    _marker: PhantomData<R>,
}

impl<T: ITcpListener + 'static, R: Runtime> Server<T, R> {
    pub async fn serve(
        self,
        service_fn: impl Fn(&http::Request) -> Result<http::Response, Box<dyn StdError + Send>>
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

                let request = http::Request::Raw(String::from_utf8_lossy(&buf).to_string());
                let ret = service_fn(&request);

                match ret {
                    Ok(response) => {
                        if let Err(e) = stream.write(response.as_bytes()).await {
                            log::error!("send response error: {:?}", e);
                        }
                    }
                    Err(e) => {
                        log::error!("handle request {:?} \nerror: {:?}", request, e);
                    }
                }
            });
        }
    }
}
