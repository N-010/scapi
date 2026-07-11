use anyhow::Result;
use scapi::bob::BobClient;

#[tokio::main]
async fn main() -> Result<()> {
    let client = BobClient::with_base_url("http://localhost:40420");
    let value = client.qubic_get_logs_by_id_range(0, 0, 100).await?;
    println!("{}", value);
    Ok(())
}
