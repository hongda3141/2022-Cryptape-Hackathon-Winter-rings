use jsonrpc_http_server::{Server, ServerBuilder};

fn main() {
    let mut server = ServerBuilder::new(8888);
    let health_check = MyHealthCheck::new();
    server = server.health_api(health_check);
    let server = server.start();
}
