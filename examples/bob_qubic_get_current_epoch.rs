use anyhow::Result;
use scapi::bob::BobRpcClient;

#[tokio::main]
async fn main() -> Result<()> {
    let client = BobRpcClient::with_base_url("http://localhost:40420");
    let value = client.qubic_get_current_epoch().await?;
    println!("{}", value);
    Ok(())
}
