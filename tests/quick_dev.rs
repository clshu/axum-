#![allow(unused)]

use anyhow::Result;
use serde_json::json;

const ADDR: &str = "http://localhost:3000";

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client(ADDR)?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "password": "web"
        }),
    );

    req_login.await?.print().await?;

    hc.do_get("/hello2/Mike").await?.print().await?;

    let req_create_ticket = hc.do_post(
        "/api/tickets",
        json!({
            "title": "AAA"
        }),
    );

    req_create_ticket.await?.print().await?;

    let req_list_tickets = hc.do_get("/api/tickets");

    req_list_tickets.await?.print().await?;

    let req_delete_ticket = hc.do_delete("/api/tickets/0");

    req_delete_ticket.await?.print().await?;

    let req_delete_ticket = hc.do_delete("/api/tickets/0");

    req_delete_ticket.await?.print().await?;

    Ok(())
}
