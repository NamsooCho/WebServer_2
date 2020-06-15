use std::convert::TryInto;
use std::io::{BufRead, BufReader, Read, Write};
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

// TODO: move to config
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
        println!("[log] start parsing http header");
        let raw_request_header = match self.parse_request_header() {
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
        println!("[log] http header parse result:\n{:#?}", request_header);

        // 1-1. if method is post
        // TODO: make Body struct
        if  request_header.is_post() {
            match request_header.get_content_length() {
                Some(content_length) if content_length > 0 => {
                    println!("[debug] content length: {}", content_length);
                    let mut body_buffer = vec![0_u8; content_length];
                    let mut offset = 0_usize;

                    // TODO: make maximum content size settable
                    loop {
                        let nbytes = self.buf_reader.read(&mut body_buffer).unwrap();
                        offset += nbytes;
                        println!("receive {} bytes. total {}", nbytes, offset);

                        if nbytes == 0 || offset >= content_length{
                            break;
                        }
                    }

                    println!("{}, {}", body_buffer.len(), offset);

                }
                _ => {}
            }
        }
        // 2. parse url
        // 3. find the Route for url
        // 4. execute handler
        // 5. response to the client

        // TODO: delete me. response for test.
        {
            let response_string = String::from(request_header);
            println!("[debug] send response\n{}", response_string);
            let content_len_head = format!("Content-Length: {}\r\n", response_string.as_bytes().len());
            self.buf_reader.get_mut().write("HTTP/1.1 200 OK\r\n".as_bytes()).unwrap();
            self.buf_reader.get_mut().write(content_len_head.as_bytes()).unwrap();
            self.buf_reader.get_mut().write("Content-Type: text/html\r\n\r\n".as_bytes()).unwrap();
            self.buf_reader.get_mut().write(response_string.as_bytes()).unwrap();
            self.buf_reader.get_mut().flush().unwrap();
            println!("[debug] task done");
        }
    }


    // TODO: move to Header struct
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

