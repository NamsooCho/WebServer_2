use std::convert::TryInto;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

use crate::http::{HttpParseError, HttpRequestHeader};
use crate::worker::task;

pub struct HttpTask {
    buf_reader: BufReader<TcpStream>,
    reader_cursor: u64,
}

impl <'a> task::Task for HttpTask {
    fn execute(&mut self) {
        self.handle_connection();
    }
}

const MAX_HEADER_SIZE: usize = 80_000;    // 80KB

impl HttpTask {
    pub fn new(stream: TcpStream) -> HttpTask {
        HttpTask {
            buf_reader: BufReader::new(stream),
            reader_cursor: 0,
        }
    }

    fn handle_connection(&mut self) {

        // TODO
        // 1. parse http header
        let raw_request_header = match self.parse_request_header() {
            Ok(header) => header,
            Err(error) => {
                eprintln!("{:?}", error);
                return;
            }
        };

        let request_header: HttpRequestHeader = match raw_request_header.try_into() {
            Ok(request_header) => request_header,
            Err(error) => {
                eprintln!("{:?}", error);
                return;
            }
        };

        // 1-1. if method is post
        // 2. parse url
        // 3. find the Route for url
        // 4. execute handler
        // 5. response to the client

        // TODO: delete me. response for test.
        {
            self.buf_reader.get_mut().write("HTTP/1.1 200 OK\r\n".as_bytes()).unwrap();
            self.buf_reader.get_mut().write("Content-Type: text/html\r\n\r\n".as_bytes()).unwrap();
            self.buf_reader.get_mut().write(String::from(request_header).as_bytes()).unwrap();
            self.buf_reader.get_mut().flush().unwrap();
        }
    }


    fn parse_request_header(&mut self) -> Result<Vec<u8>, HttpParseError> {
        let mut buffer = vec![0_u8;1024];
        let mut header_size: usize = 0;

        loop {
            let result = self.buf_reader.read_until(b'\n', &mut buffer);
            match result {
                Ok(nbytes) => {
                    header_size += nbytes;
                    if nbytes == 2 {
                        break;
                    }
                    if header_size >= MAX_HEADER_SIZE {
                        return Err(HttpParseError::ExceedCapacity);
                    }
                },
                _ => break,
            }
        }

        // CHECK_ME: Isn't there a better way?
        self.reader_cursor = header_size as u64;
        Ok(buffer[buffer.len() - header_size..].to_vec())
    }
}

