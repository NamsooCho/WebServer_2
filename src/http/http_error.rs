#[derive(Debug)]
pub enum HttpError {
    ExceedCapacity,
    HeaderParseError,
    BodyReadError,
}
