use crate::route::{Route, Router};

// role of this struct is to gather Routes and to create Router with Routes.
// you could think this struct is useless.
// but i thought i need some temporary place to store routes and to take ownership of that.
pub struct RouterBuilder {
    routes: Option<Vec<Route>>,
}

impl RouterBuilder {
    pub fn new() -> Self {
        RouterBuilder {
            routes: None,
        }
    }

    // append new Route
    pub fn append_route(&mut self, route: Route) -> &mut Self {
        match self.routes {
            None => {
                let routes = vec![route];
                self.routes = Some(routes);
            }
            Some(ref mut routes) => routes.push(route),
        }
        self
    }

    // move Routes to the Router
    pub fn build(&mut self) -> Router {
        match self.routes.take() {
            Some(routes) => Router::new(routes),
            None => Router::default(),
        }
    }
}
