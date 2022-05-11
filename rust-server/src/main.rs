#[tokio::main]
async fn main() {
    let call_server = jsonrpsee::http_server::HttpServerBuilder::default()
        .build("127.0.0.1:3000".parse::<std::net::SocketAddr>().unwrap())
        .await
        .unwrap();

    let mut module = jsonrpsee::http_server::RpcModule::new(());
    module
        .register_method("hello", |params, _context| {
            println!("call hello {:?} return world", params);

            Ok("world")
        })
        .unwrap();

    let handle = call_server.start(module).unwrap();

    tokio::join!(handle);
}
