use ashina::net::{ITcpListener, ITcpStream};
use emma::net::tcp::{TcpListener, TcpStream};
use emma::Emma;
use futures03::future::BoxFuture;

#[macro_use]
extern crate ref_thread_local;
use ref_thread_local::RefThreadLocal;

ref_thread_local! {
    static managed EMMA: Emma = emma::Builder::new().build().unwrap();
}

struct TcpStreamWrapper(TcpStream);

impl ITcpStream for TcpStreamWrapper {
    fn read<'ashina>(
        &'ashina mut self,
        buf: &'ashina mut [u8],
    ) -> BoxFuture<'ashina, std::io::Result<usize>> {
        // let fut = EMMA.with(|emma: &Emma| self.0.recv(&emma, buf).unwrap());
        let fut = self.0.recv(&EMMA.borrow(), buf).unwrap();
        todo!()
    }

    fn write<'ashina>(
        &'ashina mut self,
        src: &'ashina [u8],
    ) -> BoxFuture<'ashina, std::io::Result<()>> {
        todo!()
    }
}

struct TcpListenerWrapper(TcpListener);

fn main() {
    println!("Hello, Emma!");
}
