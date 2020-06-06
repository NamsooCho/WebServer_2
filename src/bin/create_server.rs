use lite_ws;

fn main() {
    let server = lite_ws::server::ServerBuilder::new()
        .ip_addr("127.0.0.1")
        .port_num(8888)
        .mount_route()
        .build();

    server.run();
}
