#![allow(unused)]

use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080");
    hc?.do_get("/hello2/Charlie").await?.print().await?;
    Ok(())
}