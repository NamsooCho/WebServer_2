use crate::server::Server;

#[derive(Debug)]
pub struct ServerConfig {
    pub ip_addr: String,
    pub port_num: u16,
    pub thread_count: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            ip_addr: "127.0.0.1".to_string(),
            port_num: 8888,
            thread_count: 2,
        }
    }
}

#[derive(Default)]
pub struct ServerBuilder {
    server_config: ServerConfig,
}

impl ServerBuilder {
    pub fn ip_addr(mut self, ip_addr: &str) -> Self {
        self.server_config.ip_addr = ip_addr.to_string();

        self
    }

    pub fn port_num(mut self, port_num: u16) -> Self {
        self.server_config.port_num = port_num;

        self
    }

    pub fn thread_count(mut self, thread_count: u16) -> Self {
        self.server_config.thread_count = thread_count;

        self
    }

    pub fn build(self) -> Server {
        Server::new(self.server_config)
    }
}
