use anyhow::Result;
use scapi::bob::BobRpcClient;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let client = BobRpcClient::with_base_url("http://localhost:40420");
    let filter = json!({
        "identity": "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAFXIB",
        "fromTick": 0,
        "toTick": 0
    });
    let value = client.qubic_get_qu_transfers(filter).await?;
    println!("{}", value);
    Ok(())
}
