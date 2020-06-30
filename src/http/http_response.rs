use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs::File;
use std::io;
use std::io::{Cursor, Read, Write};

use crate::http::{HttpError, HttpStatus};
use crate::http::content_type::ContentType;
use crate::http::version::{HttpVersion, Protocol};

pub struct HttpResponse {
    version: HttpVersion,
    status: HttpStatus,
    content_type: Option<ContentType>,
    headers: Option<HashMap<String, String>>,
    body_length: usize,
    body: Option<Box<dyn Read>>,
}

impl HttpResponse {
    // to make it simple to make a HttpResponse with status
    pub fn new_with(status: HttpStatus) -> Self {
        HttpResponse {
            version: HttpVersion::new(Protocol::HTTP, 1, 0),
            status,
            content_type: None,
            headers: None,
            body_length: 0,
            body: None,
        }
    }
    // is it better to move the respond function to the Request struct?
    pub fn respond<W: Write>(&mut self, write: &mut W) {
        if let Err(error) = self.try_respond(write) {
            eprintln!("[error] error while respond: {:?}", error);
        }
    }

    fn try_respond<W: Write>(&mut self, write: &mut W) -> io::Result<()> {
        write.write_all(
            format!(
                "{} {}\r\n",
                self.version.to_string(),
                self.status.to_string()
            )
                .as_bytes(),
        )?;
        write.write_all(b"Connection: close\r\n")?;

        if let Some(body) = &mut self.body {
            // TODO: support chunked
            let mut content: Vec<u8> = Vec::with_capacity(self.body_length);

            while let Ok(nbytes) = body.read_to_end(&mut content) {
                if nbytes == 0 {
                    break;
                }
            }

            if let Some(content_type) = &self.content_type {
                write.write_all(
                    format!("Content-Type: {}\r\n", content_type.to_string()).as_bytes(),
                )?;
                write.write_all(format!("Content-Length: {}\r\n", content.len()).as_bytes())?;
            }
            write.write_all(b"\r\n")?;
            write.write_all(content.as_slice())?;
        }

        write.flush()?;
        Ok(())
    }
}

// to gather values for building a http response instance.
pub struct HttpResponseBuilder {
    status: Option<HttpStatus>,
    content_type: Option<ContentType>,
    headers: Option<HashMap<String, String>>,
    body: (Option<Box<dyn Read>>, usize),
    file: Option<File>,
}

impl HttpResponseBuilder {
    pub fn new() -> Self {
        HttpResponseBuilder {
            status: None,
            content_type: None,
            headers: None,
            body: (None, 0),
            file: None,
        }
    }

    pub fn set_status(mut self, status: HttpStatus) -> Self {
        self.status = Some(status);
        self
    }

    // set the content type of the body as html
    pub fn html(mut self, html: String) -> Self {
        let html_len = html.len();
        self.content_type = Some(ContentType::TEXT_HTML);
        self.body = (Some(Box::new(Cursor::new(html.into_bytes()))), html_len);
        self
    }

    // set the content type and the body
    pub fn body(mut self, content_type: ContentType, body: Vec<u8>) -> Self {
        let body_len = body.len();
        self.content_type = Some(content_type);
        self.body = (Some(Box::new(Cursor::new(body))), body_len);

        self
    }

    pub fn file(mut self, content_type: ContentType, file: File) -> Self {
        self.content_type = Some(content_type);
        self.file = Some(file);

        self
    }

    pub fn build(mut self) -> Result<HttpResponse, HttpError> {
        let status = if let Some(status) = self.status {
            status
        } else {
            HttpStatus::OK
        };

        if let Some(file) = self.file {
            if let Ok(metadata) = file.metadata() {
                let file_len = metadata.len() as usize;
                self.body = (Some(Box::new(file)), file_len);
            } else {
                return Err(HttpError::ResponseBuildError);
            }
        }

        // TODO: this code is for the test.
        Ok(HttpResponse {
            version: HttpVersion::try_from(String::from("HTTP/1.0")).unwrap(),
            status,
            content_type: self.content_type,
            headers: self.headers,
            body_length: self.body.1,
            body: self.body.0,
        })
    }
}
