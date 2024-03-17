#![allow(unused)]

use anyhow::Result;

const ADDR: &str = "http://localhost:3000";

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client(ADDR)?;

    hc.do_get("/hello").await?.print().await?;

    Ok(())
}
