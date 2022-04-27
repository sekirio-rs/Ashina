use futures03::future::BoxFuture;
use std::io;

pub trait IFile {
    fn open<P: AsRef<std::path::Path> + Send + 'static>(
        path: P,
    ) -> BoxFuture<'static, io::Result<Self>>
    where
        Self: Sized;

    fn create<P: AsRef<std::path::Path> + Send + 'static>(
        path: P,
    ) -> BoxFuture<'static, io::Result<Self>>
    where
        Self: Sized;

    fn read<'ashina>(
        &'ashina mut self,
        buf: &'ashina mut [u8],
    ) -> BoxFuture<'ashina, io::Result<usize>>;

    fn write<'ashina>(&'ashina mut self, src: &'ashina [u8]) -> BoxFuture<'ashina, io::Result<()>>;
}
