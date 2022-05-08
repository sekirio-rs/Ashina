use std::io::{self, prelude::*};
use std::net;

const BUFFER_SIZE: usize = 1024;

fn main() -> io::Result<()> {
    let listener = net::TcpListener::bind("0.0.0.0:3344")?;

    loop {
        let mut stream = listener.accept()?.0;

        std::thread::spawn(move || {
            let mut buf = [0u8; BUFFER_SIZE];

            let _n = stream.read(&mut buf)?;

            let resp = format!("HTTP/1.1 200 OK\r\nServer: {}\r\n\r\n{:?}", "Ashina", &buf);

            stream.write(resp.as_bytes())?;

            Ok::<_, io::Error>(())
        });
    }
}
