#![allow(unused)]

use anyhow::Result;
use serde_json::json;

const ADDR: &str = "http://localhost:3000";

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client(ADDR)?;

    hc.do_get("/hello2/Mike").await?.print().await?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "password": "web"
        }),
    );

    req_login.await?.print().await?;

    Ok(())
}
