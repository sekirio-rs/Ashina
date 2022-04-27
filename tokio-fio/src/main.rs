use ashina::fio::Fio;
use ashina::fs::IFile;
use ashina::runtime::Runtime;
use futures03::future::BoxFuture;
use futures03::FutureExt;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const CORES: usize = 32;
const BENCH_SIZE: usize = 1024;

struct FileWrapper(File);

impl IFile for FileWrapper {
    fn open<P: AsRef<std::path::Path> + Send + 'static>(
        path: P,
    ) -> BoxFuture<'static, std::io::Result<Self>>
    where
        Self: Sized,
    {
        async move { File::open(path).await.map(|f| FileWrapper(f)) }.boxed()
    }

    fn create<P: AsRef<std::path::Path> + Send + 'static>(
        path: P,
    ) -> BoxFuture<'static, std::io::Result<Self>>
    where
        Self: Sized,
    {
        async move { File::create(path).await.map(|f| FileWrapper(f)) }.boxed()
    }

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

struct Tokio;

impl Runtime for Tokio {
    fn spawn<T>(future: T) -> BoxFuture<'static, T::Output>
    where
        T: futures03::Future + Send + 'static,
        T::Output: Send + 'static,
    {
        tokio::spawn(future)
            .map(|output| output.expect("tokio spawn handle return error"))
            .boxed()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::time;
    // simple_logger::SimpleLogger::new().init()?;

    let start = time::Instant::now();

    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(CORES)
        .enable_io()
        .build()
        .unwrap();

    let ret = rt.block_on(async {
        Fio::<FileWrapper, Tokio, 1024>::bench("Cargo.toml", BENCH_SIZE).await?;

        Ok(())
    });

    let cost = start.elapsed().as_micros();

    println!("[tokio-fio] bench size: {}, cost: {} micros", BENCH_SIZE, cost);

    ret
}
