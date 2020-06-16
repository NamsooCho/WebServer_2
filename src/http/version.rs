use std::convert::{TryFrom, TryInto};

use crate::http::HttpParseError;

#[derive(Debug, PartialEq)]
pub enum Protocol {
    HTTP,
}

// model to save http version info
#[derive(Debug)]
pub struct HttpVersion {
    protocol: Protocol,
    major: u8,
    minor: u8,
}


impl TryFrom<String> for HttpVersion {
    type Error = HttpParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut split = value.split('/');
        let protocol = split.next();
        let version = split.next();

        if let (Some(protocol), Some(version)) = (protocol, version) {
            let protocol: Protocol = protocol.to_string().try_into()?;
            let (major, minor) = parse_version(version)?;

            Ok(HttpVersion {
                protocol,
                major,
                minor,
            })
        } else {
            Err(HttpParseError::HeaderParseError)
        }
    }
}

impl TryFrom<String> for Protocol {
    type Error = HttpParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_ref() {
            "http" => Ok(Protocol::HTTP),
            _ => Err(HttpParseError::HeaderParseError),
        }
    }
}

fn parse_version(value: &str) -> Result<(u8, u8), HttpParseError> {
    let mut split = value.split('.');
    let major = split.next();
    let minor = split.next();

    if let (Some(major), Some(minor)) = (major, minor) {
        let major = parse_num(major)?;
        let minor = parse_num(minor)?;

        Ok((major, minor))
    } else {
        Err(HttpParseError::HeaderParseError)
    }
}

fn parse_num(num_str: &str) -> Result<u8, HttpParseError> {
    if let Ok(num) = num_str.parse::<u8>() {
        Ok(num)
    } else {
        Err(HttpParseError::HeaderParseError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_version() {
        let src = "1.0";

        match parse_version(src) {
            Ok((major, minor)) => {
                assert_eq!(major, 1);
                assert_eq!(minor, 0);
            }
            Err(e) => {
                assert!(false);
            }
        }
    }

    #[test]
    fn test_parse_http_version() -> Result<(), HttpParseError> {
        let src = "HTTP/1.0".to_string();

        let http_version: HttpVersion = src.try_into()?;
        assert_eq!(http_version.protocol, Protocol::HTTP);
        assert_eq!(http_version.major, 1);
        assert_eq!(http_version.minor, 0);

        Ok(())
    }
}
