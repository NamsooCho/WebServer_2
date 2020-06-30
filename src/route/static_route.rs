use std::fs::File;

use crate::http::{ContentType, HttpRequest, HttpResponse, HttpResponseBuilder, HttpStatus};
use crate::http::method::HttpMethod;
use crate::route::route::Route;

// route for static resources.
pub struct StaticRoute {
    static_root: String,
    // directory root that actually holds static files
    path_root: String,
    // url path root to access static resources
    extension_rules: Option<Vec<String>>,   // allowed extensions for static resources
}

impl StaticRoute {
    pub fn new(
        static_root: String,
        path_root: Option<String>,
        extension_rules: Option<Vec<String>>,
    ) -> Self {
        let path_root = if let Some(path_root) = path_root {
            path_root
        } else {
            static_root.clone()
        };

        StaticRoute {
            static_root,
            path_root,
            extension_rules,
        }
    }
}

impl Route for StaticRoute {
    fn is_path_matching(&self, method: HttpMethod, pathname: &str) -> bool {
        if method != HttpMethod::GET {
            return false
        }

        if !pathname.starts_with(&self.path_root) {
            return false
        }

        if let Some(extension_rules) = &self.extension_rules {
            for rule in extension_rules.iter() {
                if pathname.ends_with(rule) {
                    return true;
                }
            }
        }

        false
    }

    fn execute(&self, http_request: HttpRequest) -> (HttpRequest, HttpResponse) {
        let req_path = http_request.get_req_path();
        let file_path = req_path
            .get_pathname()
            .replace(&self.path_root, &self.static_root);

        let file = if let Ok(file) = File::open(&file_path[1..]) {
            file
        } else {
            return (
                http_request,
                HttpResponse::new_with(HttpStatus::NOT_FOUND),
            );
        };

        (
            http_request,
            HttpResponseBuilder::new()
                .set_status(HttpStatus::OK)
                // TODO: content type mapping by extension
                .file(ContentType::new("image", "png"), file)
                .build()
                .unwrap_or(HttpResponse::new_with(HttpStatus::INTERNAL_SERVER_ERROR)),
        )
    }
}
