#![allow(unused)]

use std::net::SocketAddr;
use axum::response::{Html, IntoResponse};
use axum::{Router, ServiceExt};
use axum::extract::{Path, Query};
use axum::routing::get;
use axum_server::Server;
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route("/hello",
    get(handler_hello)).route("/hello2/:name",get(hello_handler2));

    let addr = SocketAddr::from(([127,0,0,1],8080));
    println!("->> Listening on {addr}\n");
    Server::bind(addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();
}

#[derive(Debug,Deserialize)]
struct HelloParams {
    name: Option<String>
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}","HANDLER");

    let name = params.name.as_deref().unwrap_or("world");

    Html(format!("Hello {name}..."))
}

async fn hello_handler2(Path(name): Path<String>) -> impl IntoResponse {

    println!("->> {:<12} - handler_hello - {name:?}","HANDLER");

    Html(format!("Hello {name}..."))
}