#![allow(unused)]

use std::net::SocketAddr;
use axum::response::Html;
use axum::{Router, ServiceExt};
use axum::routing::get;
use axum_server::Server;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route("/hello",
    get(|| async {Html("Hello world...")}));

    let addr = SocketAddr::from(([127,0,0,1],8080));
    println!("->> Listening on {addr}\n");
    Server::bind(addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();
}
