use crate::utils::stopped;
use std::net::SocketAddr;
use tokio::{
    io::AsyncWriteExt as _,
    net::{TcpListener, TcpStream},
    sync::mpsc,
};
use tokio_util::sync::CancellationToken;

pub struct Listener {
    bind: (String, u16),
    stop: CancellationToken,
    stream_tx: mpsc::Sender<(TcpStream, SocketAddr)>,
}

#[derive(Debug, Clone)]
pub struct ListenerOptions {
    pub stream_tx: mpsc::Sender<(TcpStream, SocketAddr)>,
    pub bind_host: String,
    pub bind_port: u16,
}

impl Listener {
    pub fn new(options: ListenerOptions) -> Listener {
        Listener {
            stream_tx: options.stream_tx,
            bind: (options.bind_host, options.bind_port),
            stop: CancellationToken::new(),
        }
    }

    pub async fn run(mut self) -> Result<(), anyhow::Error> {
        let accept_thread_handle = tokio::spawn(async move { self.accept_thread().await });

        accept_thread_handle.await??;

        Ok(())
    }

    async fn accept_thread(&mut self) -> anyhow::Result<()> {
        let listener = TcpListener::bind(format!("{}:{}", self.bind.0, self.bind.1)).await?;
        loop {
            match stopped(&self.stop, listener.accept()).await {
                Err(_) => break,
                Ok(res) => {
                    let (socket, addr) = res?;
                    match stopped(&self.stop, self.stream_tx.send((socket, addr))).await {
                        Ok(send_res) => {
                            send_res?;
                        }
                        Err(_) => break,
                    }
                }
            }
        }
        Ok(())
    }

    async fn client_thread(
        mut socket: TcpStream,
        addr: SocketAddr,
        stop: CancellationToken,
    ) -> anyhow::Result<()> {
        println!("Got connection from {:?}", addr);
        socket.write(b"Hello, world!\n").await?;

        let mut byte_buf = vec![];
        let mut str_buf = "".to_owned();
        loop {
            tokio::select! {
                _ = stop.cancelled() => {
                    break
                }
                res = socket.readable() => {
                    res?;
                    let mut buf = [0u8; 4096];
                    match socket.try_read(&mut buf) {
                        Err(ref e) if e.kind() == tokio::io::ErrorKind::WouldBlock => {
                            continue;
                        }
                        Ok(n) => {
                            println!("Got {n} bytes from {addr:?}");
                            if n == 0 {
                                break
                            }
                            byte_buf.extend_from_slice(&buf[0..n]);

                            byte_buf = match String::from_utf8(byte_buf) {
                                Ok(s) => {
                                    str_buf.push_str(&s);
                                    vec![]
                                }
                                Err(e) => {
                                    e.into_bytes()
                                }
                            };

                            println!("String so far: {str_buf:?}");
                            if str_buf.contains("STOP") {
                                println!("Stopping!");
                                stop.cancel()
                            }
                        },
                        Err(e) => return Err(e.into()),
                    }
                }
            }
        }

        println!("Closing connection for {:?}", addr);
        socket.shutdown().await?;
        Ok(())
    }
}
