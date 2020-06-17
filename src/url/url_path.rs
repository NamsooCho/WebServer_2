use std::convert::{TryFrom, TryInto};

use regex::Regex;

use lazy_static::lazy_static;

use crate::http::HttpRequestHeader;
use crate::url::url_error::UrlError;

#[derive(Debug)]
pub struct UrlPath {
    raw: String,
    pathname: String,
    query: Option<String>,
    hash: Option<String>,
}

lazy_static! {
    static ref PATH_REGEX: Regex =
        Regex::new(r"([^?#\s]+)(?:\?([^?#\s]+))?(?:#([^\s]+))?").unwrap();
}

impl TryFrom<&str> for UrlPath {
    type Error = UrlError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        parse_path(value)
    }
}

impl TryFrom<&HttpRequestHeader> for UrlPath {
    type Error = UrlError;

    fn try_from(header: &HttpRequestHeader) -> Result<Self, Self::Error> {
        header.get_req_url().try_into()
    }
}

fn parse_path(path_str: &str) -> Result<UrlPath, UrlError> {
    let regex = Regex::new(r"([^?#\s]+)(?:\?([^?#\s]+))?(?:#([^\s]+))?").unwrap();

    let captures = if let Some(captures) = regex.captures(path_str) {
        captures
    } else {
        return Err(UrlError::PathParseError(
            "path does not match with path pattern",
        ));
    };

    let pathname = if let Some(pathname) = captures.get(1) {
        pathname.as_str().to_string()
    } else {
        return Err(UrlError::PathParseError(
            "can't found pathname from the path",
        ));
    };

    let query = if let Some(query) = captures.get(2) {
        Some(query.as_str().to_string())
    } else {
        None
    };

    let hash = if let Some(hash) = captures.get(3) {
        Some(hash.as_str().to_string())
    } else {
        None
    };

    let raw = path_str.to_string();

    Ok(UrlPath {
        raw,
        pathname,
        query,
        hash,
    })
}

#[cfg(test)]
mod tests {
    use crate::http::HttpError;

    use super::*;

    #[test]
    fn test_regex() {
        let src = r"/abc/def_123/?q1=123&q2=456#hash";

        let captures = PATH_REGEX.captures(src).unwrap();
        assert_eq!(captures.get(0).unwrap().as_str(), src);
        assert_eq!(captures.get(1).unwrap().as_str(), "/abc/def_123/");
        assert_eq!(captures.get(2).unwrap().as_str(), "q1=123&q2=456");
        assert_eq!(captures.get(3).unwrap().as_str(), "hash");

        let src = r"/abc/def_123";
        let captures = PATH_REGEX.captures(src).unwrap();
        assert_eq!(captures.get(0).unwrap().as_str(), src);
        assert_eq!(captures.get(1).unwrap().as_str(), "/abc/def_123");
        assert_eq!(captures.get(2), None);
        assert_eq!(captures.get(3), None);
    }

    #[test]
    fn test_parse_path() {
        let src = r"/abc/def-123/?q1=123&q2=456#hash";

        let result = parse_path(src).unwrap();
        assert_eq!(result.raw, src);
        assert_eq!(result.pathname, "/abc/def-123/");
        assert_eq!(result.query.unwrap(), "q1=123&q2=456");
        assert_eq!(result.hash.unwrap(), "hash");
    }

    #[test]
    fn test_parse_path_only_query() {
        let src = r"/abc/def-123/?q1=123&q2=456";

        let result = parse_path(src).unwrap();
        assert_eq!(result.raw, src);
        assert_eq!(result.pathname, "/abc/def-123/");
        assert_eq!(result.query.unwrap(), "q1=123&q2=456");
        assert_eq!(result.hash, None);
    }

    #[test]
    fn test_parse_path_only_hash() {
        let src = r"/abc/def-123/#hash";

        let result = parse_path(src).unwrap();
        assert_eq!(result.raw, src);
        assert_eq!(result.pathname, "/abc/def-123/");
        assert_eq!(result.query, None);
        assert_eq!(result.hash.unwrap(), "hash");
    }

    #[test]
    fn test_parse_path_pathname_only() {
        let src = r"/abc/def-123";

        let result = parse_path(src).unwrap();
        assert_eq!(result.raw, src);
        assert_eq!(result.pathname, "/abc/def-123");
        assert_eq!(result.query, None);
        assert_eq!(result.hash, None);
    }

    #[test]
    fn test_from_http_request_header() {
        let http_request_header: HttpRequestHeader =
            b"GET /abc/def-123/?q1=123&q2=456#hash HTTP/1.1\r\n"
                .to_vec()
                .try_into()
                .unwrap();
        let url_path: UrlPath = (&http_request_header).try_into().unwrap();

        assert_eq!(url_path.pathname, "/abc/def-123/");
        assert_eq!(url_path.query.unwrap(), "q1=123&q2=456");
        assert_eq!(url_path.hash.unwrap(), "hash");
    }
}
