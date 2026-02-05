use anyhow::Result;
use scapi::bob::BobRpcClient;

#[tokio::main]
async fn main() -> Result<()> {
    let client = BobRpcClient::with_base_url("http://localhost:40420");
    let value = client
        .qubic_broadcast_transaction("0x...signed_tx_hex...")
        .await?;
    println!("{}", value);
    Ok(())
}
