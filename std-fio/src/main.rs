use std::fs;
use std::io;
use std::io::Read;
use std::io::Write;
use std::time;

const CORES: usize = 32;
const BENCH_SIZE: usize = 1024 * 32;
const BUFFER_LEN: usize = 1024;

fn main() -> io::Result<()> {
    let start = time::Instant::now();

    let mut handles = Vec::new();

    for _ in 0..CORES {
        let h = std::thread::spawn(move || {
            let bench_size = BENCH_SIZE / CORES;

            for _ in 0..bench_size {
                let mut buf = [0u8; BUFFER_LEN];

                let mut src = fs::File::open("../LICENSE")?;

                let _n = src.read(&mut buf)?;

                let mut dev_null = fs::File::create("/dev/null")?;

                let _ = dev_null.write(&buf)?;
            }

            Ok::<_, io::Error>(())
        });

        handles.push(h);
    }

    for handle in handles {
        let _ = handle.join().unwrap();
    }

    println!(
        "[std-fio] bench_size: {}, cost: {} micros",
        BENCH_SIZE,
        start.elapsed().as_micros()
    );

    Ok(())
}
