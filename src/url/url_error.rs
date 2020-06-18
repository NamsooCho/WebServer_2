#[derive(Debug)]
pub enum UrlError {
    PathParseError(&'static str),
}
