/// model for http request header
pub struct HttpRequestHeader {
    _raw: String,
}

impl From<String> for HttpRequestHeader {
    fn from(raw: String) -> Self {
        HttpRequestHeader {
            _raw: raw
        }
    }
}

impl From<HttpRequestHeader> for String {
    fn from(req: HttpRequestHeader) -> Self {
        String::clone(&req._raw)
    }
}