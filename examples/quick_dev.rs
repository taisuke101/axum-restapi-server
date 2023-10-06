#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/hello2/Taisuke").await?.print().await?;

    // hc.do_post(
    //     "/api/login",
    //     json!({"username": "taisuke", "password": "password"}),
    // )
    // .await?
    // .print()
    // .await?;

    let req_create_ticket = hc.do_post(
        "/api/tickets",
        json!({
            "title": "TicketAAA"
        }),
    );
    req_create_ticket.await?.print().await?;

    //hc.do_delete("/api/tickets/1").await?.print().await?;

    hc.do_get("/api/tickets").await?.print().await?;

    Ok(())
}
