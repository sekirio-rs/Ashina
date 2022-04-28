use emma::fs::File;
use emma::Emma;

const BENCH_SIZE: usize = 1024 * 32;
const BUFFER_SIZE: usize = 1024;
const CORES: usize = 32;
const MAX_FILE: usize = 512;

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
    use std::time;

    let start = time::Instant::now();

    let mut handles = Vec::new();

    for _ in 0..CORES {
        let h = std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .build()
                .unwrap();

            if let Err(e) = rt.block_on(async move {
                let emma = emma::Builder::new().build().unwrap();
                let bench_size = BENCH_SIZE / CORES;
                let max_file = MAX_FILE / CORES;

                for _ in 0..(bench_size / max_file) {
                    let mut bufs = (0..max_file)
                        .into_iter()
                        .map(|_| [0u8; BUFFER_SIZE])
                        .collect::<Vec<[u8; BUFFER_SIZE]>>();

                    {
                        let mut files = files(&emma, "Cargo.toml", false, max_file)
                            .await
                            .expect("open files error");

                        let mut join = emma::Join::new(emma::Reactor::new(&emma));

                        for fut in File::multi_read(&mut files, &emma, &mut bufs)
                            .expect("multi_read error")
                        {
                            join.as_mut().join(fut);
                        }

                        join.await.map(|_| ()).map_err(|e| e.as_io_error())?;
                    }

                    {
                        let mut files = files(&emma, "/dev/null", true, max_file)
                            .await
                            .expect("create file error");

                        let mut join = emma::Join::new(emma::Reactor::new(&emma));

                        for fut in
                            File::multi_write(&mut files, &emma, &bufs).expect("multi_write error")
                        {
                            join.as_mut().join(fut);
                        }

                        join.await.map(|_| ()).map_err(|e| e.as_io_error())?;
                    }
                }

                Ok(())
            }) {
                let e: std::io::Error = e;
                eprintln!("io error: {:?}", e);
            }
        });

        handles.push(h);
    }

    for h in handles {
        h.join().unwrap();
    }

    let cost = start.elapsed().as_micros();

    println!(
        "[emma-fio] bench size: {}, cost: {} micros",
        BENCH_SIZE, cost
    );

    Ok(())
}
