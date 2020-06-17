use std::convert::TryInto;

use crate::http::{HttpError, HttpRequestBody, HttpRequestHeader};
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
}
