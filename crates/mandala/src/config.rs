use std::net::SocketAddr;

use crate::{error::ConfigError, MandalaResult};

#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub addr: SocketAddr,
}

#[derive(Debug, Default, Clone)]
pub struct ClientConfigBuilder {
    addr: Option<SocketAddr>,
}

impl ClientConfigBuilder {
    pub fn new() -> ClientConfigBuilder {
        ClientConfigBuilder { addr: None }
    }

    pub fn server_addr(&mut self, addr: SocketAddr) -> &mut ClientConfigBuilder {
        self.addr = Some(addr);
        self
    }

    pub fn build(&self) -> MandalaResult<ClientConfig> {
        let addr = self.addr.ok_or(ConfigError::MissingServerAddr)?;

        Ok(ClientConfig { addr })
    }
}

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub addr: SocketAddr,
}

#[derive(Debug, Clone)]
pub struct ServerConfigBuilder {
    addr: Option<SocketAddr>,
}

impl ServerConfigBuilder {
    pub fn new() -> ServerConfigBuilder {
        ServerConfigBuilder { addr: None }
    }

    pub fn server_addr(&mut self, addr: SocketAddr) -> &mut ServerConfigBuilder {
        self.addr = Some(addr);
        self
    }

    pub fn build(&mut self) -> MandalaResult<ServerConfig> {
        let addr = self.addr.ok_or(ConfigError::MissingServerAddr)?;

        Ok(ServerConfig { addr })
    }
}
