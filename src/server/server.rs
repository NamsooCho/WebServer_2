use std::io::Error;
use std::net::TcpListener;

use crate::server::ServerConfig;
use crate::worker::HttpTask;
use crate::worker::worker_manager::WorkerManager;

pub struct Server {
    server_config: ServerConfig,
    worker_manager: WorkerManager,
}

impl Server {
    pub fn new(server_config: ServerConfig) -> Self {
        let thread_count = server_config.thread_count;

        Server {
            server_config,
            worker_manager: WorkerManager::new(thread_count),
        }
    }

    pub fn mount_route(self) -> Self {
        self
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
            let stream = if let Ok(stream) = stream {
                stream
            } else {
                eprintln!("[error] fail to unwrap the stream");
                continue;
            };

            let peer_addr = if let Ok(peer_addr) = stream.peer_addr() {
                peer_addr.to_string()
            } else {
                "unknown".to_string()
            };

            println!("get incoming from {}", peer_addr);

            self.worker_manager
                .request(Box::new(HttpTask::new(stream)))
                .unwrap_or_else(|_error| {
                    eprintln!(
                        "error occurs while request task from {}",
                        peer_addr
                    );
                    eprintln!("{}", _error);
                });
        }

        Ok(())
    }
}
