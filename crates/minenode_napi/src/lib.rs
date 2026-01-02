use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi(js_name = "ServerOptions", object)]
pub struct JSServerOptions {
    pub host: String,
    pub port: u16,
}

#[napi(js_name = "runServer")]
pub async fn run_server(options: JSServerOptions) -> Result<()> {
    let server_options = minenode::ServerOptions {
        bind_host: options.host.clone(),
        bind_port: options.port,
    };
    let server = minenode::Server::new(server_options);
    server
        .run()
        .await
        .map_err(|e| Error::from_reason(format!("{e:?}")))?;
    Ok(())
}

#[napi(object)]
#[derive(Debug)]
pub struct AddResult {
    pub a: i32,
    pub b: i32,
    pub result: i32,
}

#[napi]
pub fn adds(a: i32, b: i32) -> AddResult {
    let res = a + b;
    AddResult { a, b, result: res }
}
