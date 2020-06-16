use crate::http::{HttpRequestBody, HttpRequestHeader};

pub struct HttpRequest {
    header: HttpRequestHeader,
    body: Option<HttpRequestBody>,
}

impl HttpRequest {
    pub fn new(header: HttpRequestHeader, body: Option<HttpRequestBody>) -> Self {
        HttpRequest {
            header,
            body,
        }
    }
}
