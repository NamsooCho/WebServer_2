use std::convert::TryFrom;

use crate::http::HttpParseError;

#[derive(Debug, Eq, PartialEq)]
pub enum HttpMethod {
    Get,
    Post,
}

impl TryFrom<String> for HttpMethod {
    type Error = HttpParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.to_lowercase();

        match value.as_str() {
            "get" => Ok(HttpMethod::Get),
            "post" => Ok(HttpMethod::Post),
            _ => Err(HttpParseError::HeaderParseError)
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
        assert_eq!(method, HttpMethod::Post);

        let method: HttpMethod = "GET".to_string().try_into().unwrap();
        assert_eq!(method, HttpMethod::Get);
    }
}
