use anyhow::Result;
use scapi::bob::BobClient;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let client = BobClient::with_base_url("http://localhost:40420");
    let filter = json!({
        "scIndex": 0,
        "logType": 0,
        "fromTick": 0,
        "toTick": 0
    });
    let value = client.qubic_find_log_ids(filter).await?;
    println!("{}", value);
    Ok(())
}
