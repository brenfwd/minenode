#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let options = minenode::ServerOptions {
        bind_host: "0.0.0.0".to_owned(),
        bind_port: 25565,
    };
    let server = minenode::Server::new(options);
    server.run().await?;
    Ok(())
}
