use crate::http::{HttpRequest, HttpResponse, HttpResponseBuilder};
use crate::http::method::HttpMethod;
use crate::route::RoutePath;

type HandlerResult = (HttpRequest, HttpResponse);

pub struct Route {
    method: HttpMethod,
    route_path: RoutePath,
    handler: Box<dyn Fn(HttpRequest, HttpResponseBuilder) -> HandlerResult + Send + Sync>,
}

impl Route {
    fn new<F>(method: HttpMethod, path: &str, handler: F) -> Result<Route, ()>
        where
            F: Fn(HttpRequest, HttpResponseBuilder) -> HandlerResult + Send + Sync + 'static,
    {
        let route_path = if let Ok(route_path) = path.parse::<RoutePath>() {
            route_path
        } else {
            return Err(());
        };

        Ok(Route {
            method,
            route_path,
            handler: Box::new(handler),
        })
    }

    // make a new Route instance for get method
    pub fn new_get<F>(path: &str, handler: F) -> Result<Route, ()>
        where
            F: Fn(HttpRequest, HttpResponseBuilder) -> HandlerResult + Send + Sync + 'static,
    {
        Route::new(HttpMethod::GET, path, handler)
    }

    // make a new Route instance for post method
    pub fn new_post<F>(path: &str, handler: F) -> Result<Route, ()>
        where
            F: Fn(HttpRequest, HttpResponseBuilder) -> HandlerResult + Send + Sync + 'static,
    {
        Route::new(HttpMethod::POST, path, handler)
    }

    // check current Route is responsible for the method and the pathname
    pub fn is_path_matching(&self, method: HttpMethod, pathname: &str) -> bool {
        method == self.method && self.route_path.is_match(pathname)
    }

    // execute request handler
    pub fn execute_handler(&self, http_request: HttpRequest) -> HandlerResult {
        self.handler.as_ref()(http_request, HttpResponseBuilder::new())
    }
}

#[cfg(test)]
mod tests {
    use crate::http::HttpStatus;

    use super::*;

    #[test]
    fn test_is_path_matching() {
        let route = Route::new_get("/test", |req, builder| {
            (
                req,
                builder.set_status(HttpStatus::NOT_FOUND).build().unwrap(),
            )
        })
            .unwrap();
        assert!(route.is_path_matching(HttpMethod::GET, "/test"));
        assert!(!route.is_path_matching(HttpMethod::POST, "/test"));
        assert!(!route.is_path_matching(HttpMethod::GET, "/example"));

        let route = Route::new_post("/test", |req, builder| {
            (req, builder.set_status(HttpStatus::OK).build().unwrap())
        })
            .unwrap();
        assert!(!route.is_path_matching(HttpMethod::GET, "/test"));
        assert!(route.is_path_matching(HttpMethod::POST, "/test"));
        assert!(!route.is_path_matching(HttpMethod::POST, "/example"));
    }
}
