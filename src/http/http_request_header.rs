use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};

use crate::http::HttpParseError;
use crate::http::method::HttpMethod;

/// model for http request header
#[derive(Debug)]
pub struct HttpRequestHeader {
    method: HttpMethod,
    req_url: String,
    version: String,
    headers: HashMap<String, String>,
}

impl TryFrom<Vec<u8>> for HttpRequestHeader {
    type Error = HttpParseError;

    fn try_from(raw: Vec<u8>) -> Result<Self, Self::Error> {
        let raw_slice = raw.as_slice();

        let (method, req_url, version) = parse_start_line(raw_slice)?;
        let headers = parse_header(raw_slice);
        let method: HttpMethod = method.try_into()?;

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

fn parse_start_line(raw: &[u8]) -> Result<(Method, Url, Version), HttpParseError> {
    let start_line_items = String::from_utf8_lossy(raw)
        .lines()
        .take(1)
        .map(|line| line.split(" ").map(String::from).collect::<Vec<String>>())
        .flatten()
        .collect::<Vec<String>>();

    if start_line_items.len() != 3 {
        return Err(HttpParseError::HeaderParseError);
    }

    Ok((
        start_line_items[0].to_string(),
        start_line_items[1].to_string(),
        start_line_items[2].to_string(),
    ))
}

fn parse_header(raw: &[u8]) -> HashMap<String, String> {
    let mut header_map = HashMap::<String, String>::new();

    for line in String::from_utf8_lossy(raw).lines().skip(1) {
        match line.find(":") {
            Some(index) => {
                let key = line[..index].to_string();
                // remove starting empty char that can be exist.
                let value = {
                    let mut i = index + 1;
                    loop {
                        if i + 1 < line.len() && &line[i..i+1] == " " {
                            break;
                        } else {
                            i += 1;
                        }
                    }

                    line[i..].to_string()
                };

                header_map.insert(key.to_lowercase(), value);
            }
            None => {
                continue;
            }
        }
    }

    header_map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_start_line() -> Result<(), HttpParseError>{
        let raw = Vec::from("GET /hello HTTP/1.1\r\nHost: 127.0.0.1:8888".as_bytes());
        let raw_slice = raw.as_slice();
        let (method, path, version) = parse_start_line(raw_slice)?;

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
        let raw_slice = raw.as_slice();

        let headers = parse_header(raw_slice);
        println!("header:\n{:#?}", headers);
    }
}
