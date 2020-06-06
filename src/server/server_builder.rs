use crate::server::Server;

#[derive(Debug)]
pub struct ServerConfig<'a> {
    pub ip_addr: &'a str,
    pub port_num: u16,
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

    pub fn port_num(mut self, port_num: u16) -> Self {
        self.server_config.port_num = port_num;

        self
    }

    pub fn mount_route(mut self) -> Self {
        self
    }

    pub fn build(mut self) -> Server<'a> {
        // why is it possible?
        Server::new(self.server_config)
    }
}
