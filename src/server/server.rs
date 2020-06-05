use crate::server::ServerConfig;

pub struct Server<'a> {
    server_config: ServerConfig<'a>
}

impl <'a> Server<'a> {
    pub fn new(server_config: ServerConfig<'a>) -> Self {
        Server {
            server_config
        }
    }

    pub fn run(&self) {
        println!("I'm running on {:?}", self.server_config);
    }
}
