use std::convert::TryInto;
use std::io::{BufReader, Error, Read};
use std::net::TcpStream;
use std::sync::Arc;
use std::time::Duration;

use crate::http::{
    HttpError, HttpRequest, HttpRequestBody, HttpRequestHeader, HttpResponseBuilder, HttpStatus,
};
use crate::route::Router;
use crate::worker::task;

pub struct HttpTask {
    buf_reader: TcpStream,
    router: Arc<Router>,
}

impl task::Task for HttpTask {
    fn execute(&mut self) {
        self.handle_connection();
    }
}

// TODO: move to config
const MAX_HEADER_SIZE: usize = 80_000; // 80KB

impl HttpTask {
    pub fn new(stream: TcpStream, router: Arc<Router>) -> Result<HttpTask, Error> {
        // delete because of the WouldBlock error on chromium base browsers
        // stream.set_read_timeout(Some(Duration::from_millis(1000)))?;
        Ok(HttpTask {
            buf_reader: stream,
            router,
        })
    }

    fn handle_connection(&mut self) {
        match self.make_http_request() {
            Ok(http_request) => {
                // find the Route for url, and execute handler.
                let (_, mut http_response) = self.router.execute_route(http_request);
                // response to the client
                http_response.respond(&mut self.buf_reader);
            }
            Err(error) => {
                if let Ok(mut http_response) = HttpResponseBuilder::new()
                    .set_status(HttpStatus::BAD_REQUEST)
                    .build()
                {
                    println!("try to send response");
                    http_response.respond(&mut self.buf_reader);
                } else {
                    // what should i do?
                    eprintln!("[error] error occurs while building response: {:?}", error);
                }
            }
        }
    }

    fn make_http_request(&mut self) -> Result<HttpRequest, HttpError> {
        // parse http header
        let raw_request_header = self.get_raw_request_header()?;
        let request_header: HttpRequestHeader = raw_request_header.try_into()?;

        // get body content if method is post
        let request_body = match request_header.get_content_length() {
            Some(content_length) if content_length > 0 => Some(HttpRequestBody::new(
                self.get_raw_request_body(content_length)?,
            )),
            _ => None,
        };

        Ok(HttpRequest::new(request_header, request_body)?)
    }

    fn get_raw_request_header(&mut self) -> Result<Vec<u8>, HttpError> {
        let mut header = vec![0_u8; 1024];
        let mut header_size: usize = 0;

        let stream = &mut self.buf_reader;
        let mut buffer = [0u8; 1];
        let mut last_new_line_index: usize = 0;

        loop {
            match stream.read_exact(&mut buffer) {
                Ok(()) => {
                    header.push(buffer[0]);
                    header_size += 1;

                    if buffer[0] == b'\n' {
                        let gap_with_last = header_size - last_new_line_index;
                        if gap_with_last == 1 || gap_with_last == 2 {
                            break;
                        } else {
                            last_new_line_index = header_size;
                        }
                    }
                }
                Err(error) => {
                    eprintln!(
                        "[error] error while read stream: {:?}\n{}",
                        error,
                        String::from_utf8(header.clone()).unwrap()
                    );
                    return Err(HttpError::ReadStreamError);
                }
            }

            if header_size >= MAX_HEADER_SIZE {
                return Err(HttpError::ExceedCapacity);
            }
        }

        Ok(header[header.len() - header_size..].to_vec())
    }

    fn get_raw_request_body(&mut self, content_length: usize) -> Result<Vec<u8>, HttpError> {
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
                return Err(HttpError::BodyReadError);
            };
        }

        Ok(body_buffer)
    }
}
