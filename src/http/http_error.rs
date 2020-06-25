#[derive(Debug)]
pub enum HttpError {
    ReadStreamError,
    ExceedCapacity,
    HeaderParseError,
    BodyReadError,
    ResponseBuildError,
}
