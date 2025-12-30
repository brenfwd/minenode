use std::net::SocketAddr;

use anyhow::Result;
use thiserror::Error;
use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
    task::JoinHandle,
};
use tokio_util::sync::CancellationToken;

pub struct Server {
    bind: (String, u16),
    stop: CancellationToken,
    client_handles: Vec<JoinHandle<Result<()>>>,
}

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("async I/O error")]
    BindError(#[from] tokio::io::Error),
    #[error("task join error")]
    JoinError(#[from] tokio::task::JoinError),
}

impl Server {
    pub async fn run(mut self) -> Result<(), ServerError> {
        let accept_thread_handle = tokio::spawn(async move { self.accept_thread().await });

        // Block until server is done
        accept_thread_handle.await??;

        Ok(())
    }

    async fn accept_thread(&mut self) -> Result<(), ServerError> {
        let listener = TcpListener::bind(format!("{}:{}", self.bind.0, self.bind.1)).await?;
        loop {
            tokio::select! {
                _ = self.stop.cancelled() => {
                    break
                }
                res = listener.accept() => {
                    let (socket, addr) = res?;
                    let stop_clone = self.stop.clone();
                    let client_handle = tokio::spawn(async move { Server::client_thread(socket, addr, stop_clone).await });
                    self.client_handles.push(client_handle);
                }
            }
        }
        Ok(())
    }

    async fn client_thread(
        mut socket: TcpStream,
        addr: SocketAddr,
        stop: CancellationToken,
    ) -> Result<()> {
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

#[derive(Default)]
pub struct ServerBuilder {
    host: Option<String>,
    port: Option<u16>,
}

#[derive(Error, Debug)]
pub enum ServerBuilderError {
    #[error("Missing required property `{0}`")]
    MissingProperty(String),
}

macro_rules! sb_prop_unwrap {
    ($obj:ident.$name:ident) => {
        $obj.$name
            .clone()
            .ok_or(ServerBuilderError::MissingProperty(
                stringify!($name).to_owned(),
            ))
    };
}

impl ServerBuilder {
    pub fn new() -> ServerBuilder {
        ServerBuilder::default()
    }

    pub fn with_bind(mut self, host: String, port: u16) -> Self {
        self.host = Some(host);
        self.port = Some(port);
        self
    }

    pub fn build(self) -> Result<Server, ServerBuilderError> {
        let server = Server {
            bind: (sb_prop_unwrap!(self.host)?, sb_prop_unwrap!(self.port)?),
            stop: CancellationToken::new(),
            client_handles: vec![],
        };
        Ok(server)
    }
}
