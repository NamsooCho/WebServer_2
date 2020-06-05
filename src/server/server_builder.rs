use crate::server::Server;

#[derive(Debug)]
pub struct ServerConfig<'a> {
    ip_addr: &'a str,
    port_num: u32,
}

impl <'a> Default for ServerConfig<'a> {
    fn default() -> Self {
        ServerConfig {
            ip_addr: "127.0.0.1",
            port_num: 8888,
        }
    }
}

pub struct ServerBuilder<'a> {
    server_config: ServerConfig<'a>,
}

impl <'a> ServerBuilder<'a> {
    pub fn new() -> Self {
        ServerBuilder {
            server_config: ServerConfig::default(),
        }
    }

    pub fn ip_addr(mut self, ip_addr: &'a str) -> Self {
        self.server_config.ip_addr = ip_addr;

        self
    }

    pub fn port_num(mut self, port_num: u32) -> Self {
        self.server_config.port_num = port_num;

        self
    }

    pub fn mount_router(mut self) -> Self {
        self
    }

    pub fn build(mut self) -> Server<'a> {
        Server::new(self.server_config)
    }
}
