use crate::http::{HttpRequest, HttpResponse, HttpResponseBuilder, HttpStatus};
use crate::route::route::Route;

// find a route and execute route's handler.
pub struct Router {
    routes: Vec<Box<dyn Route>>,
}

impl Router {
    pub fn new(routes: Vec<Box<dyn Route>>) -> Self {
        Router { routes }
    }

    // TODO list:
    // - [wip] dynamic route
    //  - get route
    //  - post route
    // - [wip] static route
    // - [wip] error response
    // - support path parameter
    pub fn execute_route(&self, http_request: HttpRequest) -> (HttpRequest, HttpResponse) {
        let req_path = http_request.get_req_path();

        for route in self.routes.iter() {
            if route.is_path_matching(http_request.get_method(), req_path.get_pathname()) {
                return route.execute_handler(http_request);
            }
        }

        (
            http_request,
            HttpResponseBuilder::new()
                .set_status(HttpStatus::NOT_FOUND)
                .build()
                .unwrap(),
        )
    }
}

impl Default for Router {
    fn default() -> Self {
        Router { routes: vec![] }
    }
}
