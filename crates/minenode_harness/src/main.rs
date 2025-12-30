#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server = minenode::ServerBuilder::new()
        .with_bind("0.0.0.0".to_owned(), 25565)
        .build()?;
    server.run().await?;
    Ok(())
}
