#![allow(unused)] // For beginning only.

// use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use axum::response::{Html, IntoResponse};
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/hello", get(handler_hello));

    let addr_port = "0.0.0.0:3000";

    let listener = tokio::net::TcpListener::bind(addr_port).await.unwrap();
    println!("->> LISTENING on {addr_port}\n");
    axum::serve(listener, app).await.unwrap();

    // Handler start

    async fn handler_hello() -> impl IntoResponse {
        println!("->> {:<12} - hendler_hello", "HANDLER");

        Html("Hello, <strong> World! </string> 2")
    }

    // Handler end
}
