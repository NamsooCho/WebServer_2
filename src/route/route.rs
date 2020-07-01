use crate::http::{HttpRequest, HttpResponse};
use crate::http::method::HttpMethod;

pub type ExecutionResult = (HttpRequest, HttpResponse);

pub trait Route: Send + Sync {
    fn is_path_matching(&self, method: HttpMethod, pathname: &str) -> bool;
    fn execute(&self, http_request: HttpRequest) -> ExecutionResult;
}

