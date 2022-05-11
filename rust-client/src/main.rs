use std::collections::HashMap;

use jsonrpsee::{core::client::ClientT, rpc_params};

#[tokio::main]
async fn main() {
    let call_client = jsonrpsee::http_client::HttpClientBuilder::default()
        .build("http://127.0.0.1:3000/rpc")
        .unwrap();
    let mut map = HashMap::<&str, i32>::new();
    map.insert("1919", 810);
    let params = rpc_params!("114", 514, map);
    let result: String = call_client.request("hello", params).await.unwrap();
    println!("{}", result.as_str());
}
