use std::convert::TryInto;

use crate::http::{HttpError, HttpRequestBody, HttpRequestHeader};
use crate::http::method::HttpMethod;
use crate::url::url_path::UrlPath;

#[derive(Debug)]
pub struct HttpRequest {
    req_path: UrlPath,
    header: HttpRequestHeader,
    body: Option<HttpRequestBody>,
}

impl HttpRequest {
    pub fn new(
        header: HttpRequestHeader,
        body: Option<HttpRequestBody>,
    ) -> Result<Self, HttpError> {
        let req_path: UrlPath = if let Ok(req_path) = (&header).try_into() {
            req_path
        } else {
            return Err(HttpError::HeaderParseError);
        };

        Ok(HttpRequest {
            req_path,
            header,
            body,
        })
    }

    pub fn get_req_path(&self) -> &UrlPath {
        &self.req_path
    }

    pub fn get_method(&self) -> HttpMethod {
        if self.header.is_get() {
            HttpMethod::GET
        } else {
            HttpMethod::POST
        }
    }
}
