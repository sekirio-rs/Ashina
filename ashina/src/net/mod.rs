//! Net part of Ashina
pub mod http;

use futures03::future::BoxFuture;
use std::io;

pub trait ITcpListener {
    type TcpStream: ITcpStream + Send;
    type SocketAddr: std::fmt::Debug;
    fn bind<'ashina>(addr: &'ashina str) -> BoxFuture<'ashina, io::Result<Self>>
    where
        Self: Sized;

    fn accept<'ashina>(
        &'ashina self,
    ) -> BoxFuture<'ashina, io::Result<(Self::TcpStream, Self::SocketAddr)>>;
}

pub trait ITcpStream {
    fn read<'ashina>(
        &'ashina mut self,
        buf: &'ashina mut [u8],
    ) -> BoxFuture<'ashina, io::Result<usize>>;

    fn write<'ashina>(&'ashina mut self, src: &'ashina [u8]) -> BoxFuture<'ashina, io::Result<()>>;
}
