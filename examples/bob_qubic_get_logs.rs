use anyhow::Result;
use scapi::bob::BobRpcClient;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let client = BobRpcClient::with_base_url("http://localhost:40420");
    let filter = json!({
        "fromTick": 0,
        "toTick": 0
    });
    let value = client.qubic_get_logs(filter).await?;
    println!("{}", value);
    Ok(())
}
