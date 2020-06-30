use lite_ws::http::{ContentType, HttpRequest, HttpResponse, HttpResponseBuilder, HttpStatus};
use lite_ws::route::{ActionRoute, StaticRoute};

fn main() {
    let server = lite_ws::server::ServerBuilder::default()
        .ip_addr("127.0.0.1")
        .port_num(8888)
        .thread_count(16)
        .build();

    server
        .mount_route(StaticRoute::new(
            "/statics".to_string(),
            Some("/st".to_string()),
            Some(vec!["png".to_string()]),
        ))
        .mount_route(ActionRoute::new_get("/", root_handler).unwrap())
        .mount_route(
            ActionRoute::new_get("/hello", |req, builder| {
                let builder = builder
                    .set_status(HttpStatus::OK)
                    .body(ContentType::TEXT_PLAIN, Vec::from("you say hello"));

                (req, builder.build().unwrap())
            })
                .unwrap(),
        )
        .run()
        .expect("fail to run server");
}

fn root_handler(req: HttpRequest, res_builder: HttpResponseBuilder) -> (HttpRequest, HttpResponse) {
    let res = res_builder
        .set_status(HttpStatus::OK)
        .html(
            "
<html>
<head></head>
<body>
    <h1>This is root page</h1>
    <img src='/st/tayo.png'>
    <img src='/st/nested/n_tayo.png'>
</body>
</html>
    "
                .to_string(),
        )
        .build();

    (req, res.unwrap())
}
