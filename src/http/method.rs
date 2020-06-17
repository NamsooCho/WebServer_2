use std::convert::TryFrom;

use crate::http::HttpError;

#[derive(Debug, Eq, PartialEq)]
pub enum HttpMethod {
    GET,
    POST,
}

impl TryFrom<String> for HttpMethod {
    type Error = HttpError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.to_lowercase();

        match value.as_str() {
            "get" => Ok(HttpMethod::GET),
            "post" => Ok(HttpMethod::POST),
            _ => Err(HttpError::HeaderParseError)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use super::*;

    #[test]
    fn test_create() {
        let method: HttpMethod = "POST".to_string().try_into().unwrap();
        assert_eq!(method, HttpMethod::POST);

        let method: HttpMethod = "GET".to_string().try_into().unwrap();
        assert_eq!(method, HttpMethod::GET);
    }
}
