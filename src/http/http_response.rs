use std::collections::HashMap;
use std::convert::TryFrom;
use std::hash::Hash;
use std::io;
use std::io::Write;

use crate::http::{HttpError, HttpStatus};
use crate::http::content_type::ContentType;
use crate::http::version::HttpVersion;

pub struct HttpResponse {
    version: HttpVersion,
    status: HttpStatus,
    content_type: Option<ContentType>,
    headers: Option<HashMap<String, String>>,
    body: Option<Vec<u8>>,
}

impl HttpResponse {
    // is it better to move the respond function to the Request struct?
    pub fn respond<W: Write>(&self, write: &mut W) {
        if let Err(error) = self.try_respond(write) {
            eprintln!("[error] error while respond: {:?}", error);
        }
    }

    fn try_respond<W: Write>(&self, write: &mut W) -> io::Result<()> {
        write.write_all(
            format!(
                "{} {}\r\n",
                self.version.to_string(),
                self.status.to_string()
            )
                .as_bytes(),
        )?;
        write.write_all(b"Connection: close\r\n")?;

        if let Some(body) = &self.body {
            let body = body.as_slice();
            if let Some(content_type) = &self.content_type {
                write.write_all(
                    format!("Content-Type: {}\r\n", content_type.to_string()).as_bytes(),
                )?;
                write.write_all(format!("Content-Length: {}\r\n", body.len()).as_bytes())?;
            }
            write.write_all(b"\r\n")?;
            write.write_all(body)?;
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
    body: Option<Vec<u8>>,
}

impl HttpResponseBuilder {
    pub fn new() -> Self {
        HttpResponseBuilder {
            status: None,
            content_type: None,
            headers: None,
            body: None,
        }
    }

    pub fn set_status(mut self, status: HttpStatus) -> Self {
        self.status = Some(status);
        self
    }

    // set the content type of the body as html
    pub fn html(mut self, html: String) -> Self {
        self.body = Some(html.into_bytes());
        self.content_type = Some(ContentType::TEXT_HTML);
        self
    }

    // set the content type and the body
    pub fn body(mut self, content_type: ContentType, body: Vec<u8>) -> Self {
        self.content_type = Some(content_type);
        self.body = Some(body);

        self
    }

    pub fn build(self) -> Result<HttpResponse, HttpError> {
        let status = if let Some(status) = self.status {
            status
        } else {
            HttpStatus::OK
        };

        // TODO: this code is for the test.
        Ok(HttpResponse {
            version: HttpVersion::try_from(String::from("HTTP/1.0")).unwrap(),
            status,
            content_type: self.content_type,
            headers: self.headers,
            body: self.body,
        })
    }
}
