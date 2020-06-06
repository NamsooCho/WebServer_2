use crate::server::ServerConfig;
use std::io::{Error, Read};
use std::net::{IpAddr, SocketAddr, TcpListener};

pub struct Server<'a> {
    server_config: ServerConfig<'a>,
}

impl<'a> Server<'a> {
    pub fn new(server_config: ServerConfig<'a>) -> Self {
        Server { server_config }
    }

    pub fn run(&self) -> Result<(), Error> {
        // ref
        // https://doc.rust-lang.org/book/ch20-01-single-threaded.html
        // https://rust-lang-nursery.github.io/rust-cookbook/net/server.html
        // https://doc.rust-lang.org/std/net/struct.TcpListener.html

        println!("I'm running on {:?}\n", self.server_config);

        let ip_addr = format!(
            "{}:{}",
            self.server_config.ip_addr, self.server_config.port_num
        );
        let listener = TcpListener::bind(ip_addr)?;

        for stream in listener.incoming() {
            println!("has income");

            let mut stream = stream.unwrap();

            // FIXME: maybe, it doesn't appropriate to handle a large request.
            let mut buffer = [0; 1024];

            stream.read(&mut buffer)?;

            println!("input: {}", String::from_utf8_lossy(&buffer[..]));
        }

        Ok(())
    }
}
