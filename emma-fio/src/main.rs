use emma::fs::File;
use emma::Emma;

const BENCH_SIZE: usize = 1024;
const BUFFER_SIZE: usize = 1024;
const CORES: usize = 32;

async fn files(
    emma: &Emma,
    path: impl AsRef<std::path::Path> + Copy,
    is_create: bool,
    size: usize,
) -> std::io::Result<Vec<File>> {
    let mut join = emma::Join::new(emma::Reactor::new(emma));

    for _ in 0..size {
        let fut = if is_create {
            File::create(emma, path).map_err(|e| e.as_io_error())?
        } else {
            File::open(emma, path).map_err(|e| e.as_io_error())?
        };

        join.as_mut().join(fut);
    }

    join.await
        .map(|ret| ret.into_iter().map(|f| f.unwrap()).collect())
        .map_err(|e| e.as_io_error())
}

fn main() -> std::io::Result<()> {
    let mut handles = Vec::new();

    for _ in 0..CORES {
        let h = std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .build()
                .unwrap();

            if let Err(e) = rt.block_on(async move {
                let emma = emma::Builder::new().build().unwrap();
                let size = BENCH_SIZE / CORES;

                let mut files = files(&emma, "Cargo.toml", false, size)
                    .await
                    .expect("open files error");

                let mut bufs = (0..size)
                    .into_iter()
                    .map(|_| [0u8; BUFFER_SIZE])
                    .collect::<Vec<[u8; BUFFER_SIZE]>>();

                let mut join = emma::Join::new(emma::Reactor::new(&emma));

                for fut in File::multi_read(&mut files, &emma, &mut bufs).expect("multi_read error")
                {
                    join.as_mut().join(fut);
                }

                join.await.map(|_| ()).map_err(|e| e.as_io_error())
            }) {
                eprintln!("io error: {:?}", e);
            }
        });

        handles.push(h);
    }

    for h in handles {
        h.join().unwrap();
    }

    Ok(())
}
