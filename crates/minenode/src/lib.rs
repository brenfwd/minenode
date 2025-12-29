use thiserror::Error;

pub struct Server {
    bind: (String, u16),
}

#[derive(Default)]
pub struct ServerBuilder {
    host: Option<String>,
    port: Option<u16>,
}

#[derive(Debug, Error)]
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
        };
        Ok(server)
    }
}
