use crate::fs::IFile;
use crate::runtime::Runtime;
use std::error::Error as StdError;
use std::marker::PhantomData;

pub struct Fio<const N: usize, F: IFile, R: Runtime> {
    _marker0: PhantomData<F>,
    _marker1: PhantomData<R>,
}

impl<const N: usize, F: IFile + Send + 'static, R: Runtime> Fio<N, F, R> {
    pub async fn bench(
        path: impl AsRef<std::path::Path> + Copy + Send + Sync + 'static,
        bench_size: usize,
    ) -> Result<(), Box<dyn StdError>> {
        let mut handles = Vec::new();

        for _ in 0..bench_size {
            let handle = R::spawn(async move {
                let mut buf = [0; N];
                let mut file = F::open(path).await.expect("open file error");

                let _ = file.read(&mut buf).await.expect("read file error");

                log::info!("{:?}", String::from_utf8_lossy(&buf));

                F::create("/dev/null")
                    .await
                    .expect("create file error")
                    .write(&buf)
                    .await
                    .expect("write file error");
            });

            handles.push(handle);
        }

        for handle in handles {
            let _ = handle.await;
        }

        Ok(())
    }
}
