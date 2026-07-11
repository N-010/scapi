use anyhow::Result;
use scapi::bob::BobClient;

#[tokio::main]
async fn main() -> Result<()> {
    let client = BobClient::with_base_url("http://localhost:40420");
    let value = client.qubic_unsubscribe("SUBSCRIPTION_ID").await?;
    println!("{}", value);
    Ok(())
}
