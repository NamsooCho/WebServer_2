use std::convert::TryInto;
use std::str::FromStr;

use crate::route::RouteError;
use crate::url::url_path::UrlPath;

#[derive(Debug)]
pub struct RoutePath {
    path: String,
}

impl RoutePath {
    pub fn is_match(&self, pathname: &str) -> bool {
        *pathname == self.path
    }
}

impl FromStr for RoutePath {
    type Err = RouteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let url_path: UrlPath = if let Ok(url_path) = s.try_into() {
            url_path
        } else {
            return Err(RouteError::RoutePathParseError);
        };

        Ok(RoutePath {
            path: url_path.get_pathname().to_string(),
        })
    }
}
