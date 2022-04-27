use ashina::fio::Fio;
use ashina::fs::IFile;
use ashina::runtime::Runtime;
use async_std::fs::File;
use async_std::io::{ReadExt, WriteExt};
use futures03::future::BoxFuture;
use futures03::FutureExt;

const BENCH_SIZE: usize = 100;

struct FileWrapper(File);

impl IFile for FileWrapper {
    fn open<P: AsRef<std::path::Path> + Send + 'static>(
        path: P,
    ) -> BoxFuture<'static, std::io::Result<Self>>
    where
        Self: Sized,
    {
        async move {
            let path = path.as_ref().clone();
            File::open(path).await.map(|f| FileWrapper(f))
        }
        .boxed()
    }

    fn create<P: AsRef<std::path::Path> + Send + 'static>(
        path: P,
    ) -> BoxFuture<'static, std::io::Result<Self>>
    where
        Self: Sized,
    {
        async move {
            let path = path.as_ref().clone();
            File::create(path).await.map(|f| FileWrapper(f))
        }
        .boxed()
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

struct AsyncStd;

impl Runtime for AsyncStd {
    fn spawn<T>(future: T) -> BoxFuture<'static, T::Output>
    where
        T: futures03::Future + Send + 'static,
        T::Output: Send + 'static,
    {
        async_std::task::spawn(future).boxed()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::time;
    // simple_logger::SimpleLogger::new().init()?;

    let start = time::Instant::now();

    let ret = async_std::task::block_on(async {
        Fio::<FileWrapper, AsyncStd, 1024>::bench("Cargo.toml", BENCH_SIZE).await?;

        Ok(())
    });

    let cost = start.elapsed().as_micros();

    println!(
        "[async-fio] bench size: {}, cost: {} micros",
        BENCH_SIZE, cost
    );

    ret
}
