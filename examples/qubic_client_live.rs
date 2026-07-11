use anyhow::Result;
use scapi::QubicClient;

#[tokio::main]
async fn main() -> Result<()> {
    let client = QubicClient::new();
    let tick = client.live().get_tick_info().await?;
    println!("tick-info: {:?}", tick.tick_info);

    // Replace with a real identity if needed.
    let identity = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAFXIB";
    let balance = client.live().get_balance(identity.into()).await?;
    println!("balance: {:?}", balance.balance);

    Ok(())
}
