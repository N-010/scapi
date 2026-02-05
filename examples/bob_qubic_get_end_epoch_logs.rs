use anyhow::Result;
use scapi::bob::BobRpcClient;

#[tokio::main]
async fn main() -> Result<()> {
    let client = BobRpcClient::with_base_url("http://localhost:40420");
    let value = client.qubic_get_end_epoch_logs(0).await?;
    println!("{}", value);
    Ok(())
}
