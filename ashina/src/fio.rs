use crate::fs::IFile;
use crate::runtime::Runtime;
use std::error::Error as StdError;
use std::marker::PhantomData;

const MAX_FILE: usize = 512;

pub struct Fio<F: IFile, R: Runtime, const N: usize> {
    _marker0: PhantomData<F>,
    _marker1: PhantomData<R>,
}

impl<F: IFile + Send + 'static, R: Runtime, const N: usize> Fio<F, R, N> {
    pub async fn bench(
        path: impl AsRef<std::path::Path> + Copy + Send + Sync + 'static,
        bench_size: usize,
    ) -> Result<(), Box<dyn StdError>> {
        for _ in 0..bench_size / MAX_FILE {
            let mut handles = Vec::new();

            for _ in 0..MAX_FILE {
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
        }

        Ok(())
    }
}
