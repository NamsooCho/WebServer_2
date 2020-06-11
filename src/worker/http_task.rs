use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

use crate::http::HttpRequestHeader;
use crate::worker::task;

pub struct HttpTask {
    stream: TcpStream,
}

impl task::Task for HttpTask {
    fn execute(&mut self) {
        self.handle_connection();
    }
}

impl HttpTask {
    pub fn new(stream: TcpStream) -> HttpTask {
        HttpTask {
            stream
        }
    }

    fn handle_connection(&mut self) {

        // TODO
        // 1. parse http header
        let raw_request_header = self.parse_request_header();
        let request_header: HttpRequestHeader = raw_request_header.into();

        // 2. parse url
        // 3. find the Route for url
        // 4. execute handler
        // 5. response to the client

        // TODO: delete me. response for test.
        {
            self.stream.write("HTTP/1.1 200 OK\r\n\r\n".as_bytes()).unwrap();
            self.stream.write(String::from(request_header).as_bytes()).unwrap();
            self.stream.flush().unwrap();
        }
    }

    fn parse_request_header(&self) -> String {
        let mut reader = BufReader::new(&self.stream);
        let mut raw_request = String::new();

        loop {
            let result = reader.read_line(&mut raw_request);
            match result {
                Ok(data) => {
                    if data == 2 {
                        break;
                    }
                },
                _ => break,
            }
        }

        raw_request
    }
}

