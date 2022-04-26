use ashina::net::{ITcpListener, ITcpStream};
use ashina::runtime::Runtime;
use ashina::server::HttpServerBuilder;
use ashina::service::simple_handler;
use futures03::future::BoxFuture;
use futures03::FutureExt;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

struct TcpStreamWrapper(TcpStream);

impl ITcpStream for TcpStreamWrapper {
    fn read<'ashina>(
        &'ashina mut self,
        buf: &'ashina mut [u8],
    ) -> BoxFuture<'ashina, std::io::Result<usize>> {
        self.0.read(buf).boxed()
    }
    fn write<'ashina>(
        &'ashina mut self,
        src: &'ashina [u8],
    ) -> BoxFuture<'ashina, std::io::Result<()>> {
        async move { self.0.write(src).await.map(|_| ()) }.boxed()
    }
}

struct TcpListenerWrapper(TcpListener);

impl ITcpListener for TcpListenerWrapper {
    type TcpStream = TcpStreamWrapper;
    type SocketAddr = std::net::SocketAddr;
    fn bind<'ashina>(addr: &'ashina str) -> BoxFuture<'ashina, std::io::Result<Self>>
    where
        Self: Sized,
    {
        async move {
            TcpListener::bind(addr)
                .await
                .map(|listener| TcpListenerWrapper(listener))
        }
        .boxed()
    }

    fn accept<'ashina>(
        &'ashina self,
    ) -> BoxFuture<'ashina, std::io::Result<(Self::TcpStream, Self::SocketAddr)>> {
        async move {
            self.0
                .accept()
                .await
                .map(|(stream, addr)| (TcpStreamWrapper(stream), addr))
        }
        .boxed()
    }
}

struct Tokio;

impl Runtime for Tokio {
    fn spawn<T>(future: T)
    where
        T: futures03::Future + Send + 'static,
        T::Output: Send + 'static,
    {
        tokio::spawn(future);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // simple_logger::SimpleLogger::new().init()?;

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_io()
        .build()
        .unwrap();

    rt.block_on(async {
        let server = HttpServerBuilder::new()
            .ip("0.0.0.0")
            .port(3344)
            .build::<TcpListenerWrapper, Tokio>()
            .await?;

        server.serve(simple_handler).await?;

        Ok(())
    })
}
