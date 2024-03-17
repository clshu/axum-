#![allow(unused)] // For beginning only.

// use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use axum::response::Html;
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route(
        "/",
        get(|| async { Html("Hello, <strong> World! </string>") }),
    );

    let addr_port = "0.0.0.0:3000";
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(addr_port).await.unwrap();
    println!("->> LISTENING on {addr_port}\n");
    axum::serve(listener, app).await.unwrap();
}
