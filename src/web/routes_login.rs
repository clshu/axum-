use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

use crate::{Error, Result};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");
    // TODO: Imlement real db/autu logic.
    if payload.username != "demo1" || payload.password != "web" {
        return Err(Error::LoginFail);
    }

    // TODO: Implement real auth-token generation/signature
    cookies.add(Cookie::new("auth-token", "user-1.exp.sign"));

    // create success body
    let body = Json(json!({
        "result": {
            "success": true,
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}
