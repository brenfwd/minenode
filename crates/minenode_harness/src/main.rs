fn main() {
    let server = minenode::ServerBuilder::new()
        .with_bind("0.0.0.0".to_owned(), 25565)
        .build()
        .unwrap();
    println!("Hello, world!");
}
