use lite_ws::http::{ContentType, HttpRequest, HttpResponse, HttpResponseBuilder, HttpStatus};
use lite_ws::route::Route;

fn main() {
    let server = lite_ws::server::ServerBuilder::default()
        .ip_addr("127.0.0.1")
        .port_num(8888)
        .thread_count(16)
        .build();

    server
        .mount_route(Route::new_get("/", root_handler).unwrap())
        .mount_route(
            Route::new_get("/hello", |req, builder| {
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
</body>
</html>
    "
                .to_string(),
        )
        .build();

    (req, res.unwrap())
}
