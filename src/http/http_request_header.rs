use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};

use crate::http::HttpError;
use crate::http::method::HttpMethod;
use crate::http::version::HttpVersion;
use crate::util::lines::Lines;

/// model for http request header
#[derive(Debug)]
pub struct HttpRequestHeader {
    method: HttpMethod,
    req_url: String,
    version: HttpVersion,
    headers: HashMap<String, String>,
}

impl HttpRequestHeader {
    pub fn is_get(&self) -> bool {
        self.method == HttpMethod::GET
    }

    pub fn is_post(&self) -> bool {
        self.method == HttpMethod::POST
    }

    pub fn get_req_url(&self) -> &str {
        &self.req_url
    }

    pub fn get_content_length(&self) -> Option<usize> {
        self.get_header("content-length")
    }
}

// this Trait is to read a header value with a specific type
pub trait ReadHeaderAs<T> {
    fn get_header(&self, key: &str) -> Option<T>;
}

impl ReadHeaderAs<usize> for HttpRequestHeader {
    fn get_header(&self, key: &str) -> Option<usize> {
        if let Some(value) = self.headers.get(key) {
            if let Ok(value_as) = value.parse::<usize>() {
                Some(value_as)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl ReadHeaderAs<String> for HttpRequestHeader {
    fn get_header(&self, key: &str) -> Option<String> {
        if let Some(value) = self.headers.get(key) {
            Some(value.clone())
        } else {
            None
        }
    }
}

// to make a request header from a Vec<u8> type.
impl TryFrom<Vec<u8>> for HttpRequestHeader {
    type Error = HttpError;

    fn try_from(raw: Vec<u8>) -> Result<Self, Self::Error> {
        let mut lines = Lines::new(raw);
        let (method, req_url, version) = if let Some(start_line) = lines.next() {
            parse_start_line(start_line)?
        } else {
            return Err(HttpError::HeaderParseError);
        };

        let headers = parse_header(&mut lines)?;
        let method: HttpMethod = method.try_into()?;
        let version: HttpVersion = version.try_into()?;

        Ok(HttpRequestHeader {
            method,
            req_url,
            version,
            headers,
        })
    }
}

impl From<HttpRequestHeader> for String {
    fn from(req_header: HttpRequestHeader) -> Self {
        format!("{:#?}", req_header)
    }
}

type Method = String;
type Url = String;
type Version = String;

fn parse_start_line(first_line: String) -> Result<(Method, Url, Version), HttpError> {
    let mut split = first_line.split(' ');
    let method = split.next();
    let req_url = split.next();
    let version = split.next();

    if let (Some(method), Some(req_url), Some(version)) = (method, req_url, version) {
        Ok((method.to_string(), req_url.to_string(), version.to_string()))
    } else {
        Err(HttpError::HeaderParseError)
    }
}

fn parse_header(raw: &mut Lines) -> Result<HashMap<String, String>, HttpError> {
    let mut header_map = HashMap::<String, String>::new();

    for line in raw {
        let mut split = line.split(':');
        let key = split.next();
        let value = split.next();

        if let (Some(key), Some(value)) = (key, value) {
            header_map.insert(
                key.trim().to_lowercase().to_string(),
                value.trim().to_string(),
            );
        } else {
            continue;
        }
    }

    Ok(header_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_start_line() -> Result<(), HttpError> {
        let raw = Vec::from("GET /hello HTTP/1.1\r\nHost: 127.0.0.1:8888".as_bytes());
        let mut lines = Lines::new(raw);
        let (method, path, version) = parse_start_line(lines.next().unwrap())?;

        assert_eq!(method, "GET");
        assert_eq!(path, "/hello");
        assert_eq!(version, "HTTP/1.1");

        Ok(())
    }

    #[test]
    fn test_parse_header() {
        let raw = "GET / HTTP/1.1
Host: 127.0.0.1:8888
Connection: keep-alive
Cache-Control: max-age=0
Upgrade-Insecure-Requests: 1
User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.163 Whale/2.7.99.22 Safari/537.36";
        let raw = Vec::from(raw.as_bytes());
        let mut lines = Lines::new(raw);

        lines.next().unwrap();

        let headers = parse_header(&mut lines).unwrap();
        assert_eq!(headers.get("connection"), Some(&"keep-alive".to_string()));
    }

    #[test]
    fn test_request_header() {
        let raw = "GET / HTTP/1.1
Host: 127.0.0.1:8888
Connection: keep-alive
Cache-Control: max-age=0
Upgrade-Insecure-Requests: 1
User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.163 Whale/2.7.99.22 Safari/537.36";
        let raw = Vec::from(raw.as_bytes());

        let request_header: HttpRequestHeader = raw.try_into().unwrap();
        assert_eq!(
            request_header.get_header("upgrade-insecure-requests"),
            Some(1)
        );
        assert_eq!(
            request_header.get_header("connection"),
            Some("keep-alive".to_string())
        );
    }
}
