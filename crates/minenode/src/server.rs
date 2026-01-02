use tokio::{sync::mpsc, task::JoinHandle};
use tokio_util::sync::CancellationToken;

use crate::{Connection, Listener, ListenerOptions};

pub struct ServerOptions {
    pub bind_host: String,
    pub bind_port: u16,
}

pub struct Server {
    options: ServerOptions,
    stop: CancellationToken,
}

impl Server {
    pub fn new(options: ServerOptions) -> Server {
        let stop = CancellationToken::new();

        Server {
            options,
            stop: stop,
        }
    }

    pub async fn run(self) -> anyhow::Result<()> {
        let (tx, mut rx) = mpsc::channel(256);
        let listener = {
            let listener_options = ListenerOptions {
                stream_tx: tx,
                bind_host: self.options.bind_host.clone(),
                bind_port: self.options.bind_port,
            };
            tokio::spawn(async move {
                let listener = Listener::new(listener_options);
                listener.run().await
            })
        };

        let mut tasks: Vec<JoinHandle<Result<(), anyhow::Error>>> = vec![];

        loop {
            tokio::select! {
                _ = self.stop.cancelled() => {
                    break
                }

                Some((stream, addr)) = rx.recv() => {
                    println!("New connection from {addr:?}");
                    let connection_task = {
                        let connection = Connection::new(self.stop.clone(), stream, addr);
                        tokio::spawn(async move {
                            connection.run().await
                        })
                    };
                    tasks.push(connection_task);
                }
            }
        }

        tasks.push(listener);

        for task in tasks {
            task.await??;
        }

        Ok(())
    }
}
