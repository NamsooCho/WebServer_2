#[derive(Debug)]
pub enum HttpParseError {
    ExceedCapacity,
    HeaderParseError,
}
