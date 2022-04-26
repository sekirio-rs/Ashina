use emma::alias::*;
use emma::net::tcp::TcpListener;

const BUFFER_SIZE: usize = 1024;

fn main() -> std::io::Result<()> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();

    rt.block_on(async {
        let emma = emma::Builder::new().build().unwrap();

        let listener = TcpListener::bind("0.0.0.0:3344").unwrap();

        loop {
            let stream = accept_socket(&emma, &listener)
                .await
                .expect("accept_socket error");

            std::thread::spawn(move || {
                tokio::runtime::Builder::new_current_thread()
                    .build()
                    .unwrap()
                    .block_on(async move {
                        let emma = emma::Builder::new().build().unwrap();

                        let mut buf = [0; BUFFER_SIZE];

                        let _n = recv_msg(&emma, &mut buf, &stream)
                            .await
                            .expect("recv_msg error");

                        let resp = format!(
                            "HTTP/1.1 200 OK\r\nServer: {}\r\n\r\n{}",
                            "Ashina", "Goobye, Sekiro."
                        );

                        send_msg(&emma, resp.as_bytes(), &stream)
                            .await
                            .expect("send_msg error");
                    })
            });
        }
    })
}
