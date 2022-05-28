use emma::alias::*;
use emma::net::tcp::TcpListener;

const BUFFER_SIZE: usize = 1024;
const CORES: usize = 32;

fn main() -> std::io::Result<()> {
    let mut handles = Vec::new();

    for _ in 0..CORES {
        let h = std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .build()
                .unwrap();

            rt.block_on(async move {
                let emma = emma::Builder::new().build().unwrap();

                let listener = TcpListener::bind("0.0.0.0:3344").unwrap();

                let local = tokio::task::LocalSet::new();
                local
                    .run_until(async {
                        loop {
                            let stream = accept_socket(&emma, &listener)
                                .await
                                .expect("accept_socket error");

                            let emma_cloned = emma.clone();
                            local
                                .spawn_local(async move {
                                    let mut buf = [0; BUFFER_SIZE];

                                    let _n = recv_msg(&emma_cloned, &mut buf, &stream)
                                        .await
                                        .expect("recv_msg error");

                                    let mut svg = open_file(&emma_cloned, "github.svg")
                                        .await
                                        .expect("open github.svg error");
                                    read_file(&emma_cloned, &mut svg, &mut buf)
                                        .await
                                        .expect("read github.svg error");

                                    let resp = format!(
                                        "HTTP/1.1 200 OK\r
Accept-Ranges: bytes\r
Cache-Control: public, max-age=0\r
ETag: W/\"3c8-180fe2971b6\"\r
Content-Type: image/svg+xml\r
Content-Length: 968\r
\r\n"
                                    );

                                    send_msg(&emma_cloned, resp.as_bytes(), &stream)
                                        .await
                                        .expect("send response error");

                                    send_msg(&emma_cloned, &buf, &stream)
                                        .await
                                        .expect("send image error");
                                })
                                .await
                                .unwrap();
                        }
                    })
                    .await;
            });
        });

        handles.push(h);
    }

    handles.into_iter().for_each(|h| h.join().unwrap());
    Ok(())
}
