use crate::http::{HttpRequest, HttpResponseBuilder};
use crate::http::method::HttpMethod;
use crate::route::route::{ExecutionResult, Route};
use crate::route::RoutePath;

pub struct ActionRoute {
    method: HttpMethod,
    route_path: RoutePath,
    handler: Box<dyn Fn(HttpRequest, HttpResponseBuilder) -> ExecutionResult + Send + Sync>,
}

impl ActionRoute {
    fn new<F>(method: HttpMethod, path: &str, handler: F) -> Result<ActionRoute, ()>
        where
            F: Fn(HttpRequest, HttpResponseBuilder) -> ExecutionResult + Send + Sync + 'static,
    {
        let route_path = if let Ok(route_path) = path.parse::<RoutePath>() {
            route_path
        } else {
            return Err(());
        };

        Ok(ActionRoute {
            method,
            route_path,
            handler: Box::new(handler),
        })
    }

    // make a new Route instance for get method
    pub fn new_get<F>(path: &str, handler: F) -> Result<ActionRoute, ()>
        where
            F: Fn(HttpRequest, HttpResponseBuilder) -> ExecutionResult + Send + Sync + 'static,
    {
        ActionRoute::new(HttpMethod::GET, path, handler)
    }

    // make a new Route instance for post method
    pub fn new_post<F>(path: &str, handler: F) -> Result<ActionRoute, ()>
        where
            F: Fn(HttpRequest, HttpResponseBuilder) -> ExecutionResult + Send + Sync + 'static,
    {
        ActionRoute::new(HttpMethod::POST, path, handler)
    }
}

impl Route for ActionRoute {
    // check current Route is responsible for the method and the pathname
    fn is_path_matching(&self, method: HttpMethod, pathname: &str) -> bool {
        method == self.method && self.route_path.is_match(pathname)
    }

    // execute request handler
    fn execute(&self, http_request: HttpRequest) -> ExecutionResult {
        self.handler.as_ref()(http_request, HttpResponseBuilder::new())
    }
}

#[cfg(test)]
mod tests {
    use crate::http::HttpStatus;

    use super::*;

    #[test]
    fn test_is_path_matching() {
        let route = ActionRoute::new_get("/test", |req, builder| {
            (
                req,
                builder.set_status(HttpStatus::NOT_FOUND).build().unwrap(),
            )
        })
            .unwrap();
        assert!(route.is_path_matching(HttpMethod::GET, "/test"));
        assert!(!route.is_path_matching(HttpMethod::POST, "/test"));
        assert!(!route.is_path_matching(HttpMethod::GET, "/example"));

        let route = ActionRoute::new_post("/test", |req, builder| {
            (req, builder.set_status(HttpStatus::OK).build().unwrap())
        })
            .unwrap();
        assert!(!route.is_path_matching(HttpMethod::GET, "/test"));
        assert!(route.is_path_matching(HttpMethod::POST, "/test"));
        assert!(!route.is_path_matching(HttpMethod::POST, "/example"));
    }
}
