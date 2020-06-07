use crate::server::Server;

#[derive(Debug)]
pub struct ServerConfig {
    pub ip_addr: String,
    pub port_num: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            ip_addr: "127.0.0.1".to_string(),
            port_num: 8888,
        }
    }
}

pub struct ServerBuilder {
    server_config: ServerConfig,
}

impl ServerBuilder {
    pub fn new() -> Self {
        ServerBuilder {
            server_config: ServerConfig::default(),
        }
    }

    pub fn ip_addr(mut self, ip_addr: &str) -> Self {
        self.server_config.ip_addr = ip_addr.to_string();

        self
    }

    pub fn port_num(mut self, port_num: u16) -> Self {
        self.server_config.port_num = port_num;

        self
    }

    pub fn mount_route(mut self) -> Self {
        self
    }

    pub fn build(mut self) -> Server {
        Server::new(self.server_config)
    }
}
