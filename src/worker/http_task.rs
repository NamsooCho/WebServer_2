use std::convert::TryInto;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;

use crate::http::{HttpParseError, HttpRequest, HttpRequestBody, HttpRequestHeader};
use crate::worker::task;

pub struct HttpTask {
    buf_reader: BufReader<TcpStream>,
}

impl task::Task for HttpTask {
    fn execute(&mut self) {
        self.handle_connection();
    }
}

// TODO: move to config
const MAX_HEADER_SIZE: usize = 80_000; // 80KB

impl HttpTask {
    pub fn new(stream: TcpStream) -> HttpTask {
        HttpTask {
            buf_reader: BufReader::new(stream),
        }
    }

    fn handle_connection(&mut self) {
        // TODO
        // 1. parse http header
        let raw_request_header = match self.get_raw_request_header() {
            Ok(header) => header,
            Err(error) => {
                // TODO: route to the error page
                eprintln!("{:?}", error);
                return;
            }
        };

        let request_header: HttpRequestHeader = match raw_request_header.try_into() {
            Ok(request_header) => request_header,
            Err(error) => {
                // TODO: route to the error page
                eprintln!("{:?}", error);
                return;
            }
        };

        // 1-1. if method is post
        let request_body = match request_header.get_content_length() {
            Some(content_length) if content_length > 0 => {
                match self.get_raw_request_body(content_length) {
                    Ok(raw_body) => Some(HttpRequestBody::new(raw_body)),
                    Err(error) => {
                        // TODO: route to the error page
                        eprintln!("{:?}", error);
                        return;
                    }
                }
            }
            _ => None,
        };

        let http_request = HttpRequest::new(request_header, request_body);

        // 2. parse url
        // 3. find the Route for url
        // 4. execute handler
        // 5. response to the client

        // TODO: delete me. response for test.
        {
            self.buf_reader
                .get_mut()
                .write_all(b"HTTP/1.1 200 OK\r\n")
                .unwrap();
            self.buf_reader
                .get_mut()
                .write_all(b"Content-Type: text/html\r\n\r\n")
                .unwrap();
            self.buf_reader
                .get_mut()
                .write_all(b"success")
                .unwrap();
            self.buf_reader.get_mut().flush().unwrap();
        }
    }

    fn get_raw_request_header(&mut self) -> Result<Vec<u8>, HttpParseError> {
        let mut buffer = vec![0_u8; 1024];
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
                }
                _ => break,
            }
        }

        Ok(buffer[buffer.len() - header_size..].to_vec())
    }

    fn get_raw_request_body(&mut self, content_length: usize) -> Result<Vec<u8>, HttpParseError> {
        let mut body_buffer = vec![0_u8; content_length];
        let mut offset = 0_usize;

        // TODO: make the maximum content size settable
        loop {
            if let Ok(nbytes) = self.buf_reader.read(&mut body_buffer) {
                offset += nbytes;

                if nbytes == 0 || offset >= content_length {
                    break;
                }
            } else {
                return Err(HttpParseError::BodyReadError);
            };
        }

        Ok(body_buffer)
    }
}
