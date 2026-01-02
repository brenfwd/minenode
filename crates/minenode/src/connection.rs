use std::net::SocketAddr;

use tokio::net::TcpStream;
use tokio_util::sync::CancellationToken;

#[derive(Debug)]
pub struct Connection {
    stop: CancellationToken,
    tcp_stream: TcpStream,
    tcp_addr: SocketAddr,
}

impl Connection {
    pub fn new(stop: CancellationToken, tcp_stream: TcpStream, tcp_addr: SocketAddr) -> Connection {
        Connection {
            stop,
            tcp_stream,
            tcp_addr,
        }
    }

    pub async fn run(mut self) -> anyhow::Result<()> {
        self.stop.cancelled().await;
        Ok(())
    }
}
