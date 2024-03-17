#![allow(unused)] // For beginning only.

use axum::extract::{Path, Query};
// use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use axum::response::{Html, IntoResponse};
use axum::{routing::get, Router};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let routes_all = Router::new().merge(routes_hello());

    let addr_port = "0.0.0.0:3000";

    let listener = tokio::net::TcpListener::bind(addr_port).await.unwrap();
    println!("->> LISTENING on {addr_port}\n");
    axum::serve(listener, routes_all).await.unwrap();

    // Router start -- Route hello*
    fn routes_hello() -> Router {
        Router::new()
            .route("/hello", get(handler_hello))
            .route("/hello2/:name", get(handler_hello2))
    }
    // Router end
    // Handler with extractor start

    #[derive(Debug, Deserialize)]
    struct HelloParams {
        name: Option<String>,
    }

    // e.g., '/hello?name=Jen'
    async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
        println!("->> {:<12} - hendler_hello - {params:?}", "HANDLER");

        let name = params.name.as_deref().unwrap_or("World!");
        Html(format!("Hello, <strong>{name}</string>"))
    }

    // e.g., '/hello2/Mike
    async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
        println!("->> {:<12} - hendler_hello2 - {name:?}", "HANDLER");

        Html(format!("Hello2, <strong>{name}</string>"))
    }

    // Handler end
}
