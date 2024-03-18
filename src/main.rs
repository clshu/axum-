#![allow(unused)] // For beginning only.

pub use self::error::{Error, Result};
use axum::extract::{Path, Query};
use axum::middleware;
// use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::get_service;
use axum::{routing::get, Router};
use serde::Deserialize;
use tower_http::services::ServeDir;

mod error;
mod web;

#[tokio::main]
async fn main() {
    let routes = web::routes_login::routes();
    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(routes)
        .layer(middleware::map_response(main_response_mapper))
        .fallback_service(routes_static());

    let addr_port = "0.0.0.0:3000";

    let listener = tokio::net::TcpListener::bind(addr_port).await.unwrap();
    println!("->> LISTENING on {addr_port}\n");
    axum::serve(listener, routes_all).await.unwrap();
}

// Mapper as a middileware
async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    println!();
    res
}

// Routes start -- Route hello*
fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}
// -- Route static
// `cargo add tower-http -F fs` for ServeDir
fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

// Routes end

// Handler start

// -- extractor Query
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

// -- extractor Path
// e.g., '/hello2/Mike
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - hendler_hello2 - {name:?}", "HANDLER");

    Html(format!("Hello2, <strong>{name}</string>"))
}

// Handler end
