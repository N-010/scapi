use anyhow::Result;
use scapi::rpc::get::{get_balance, get_tick_info};

#[tokio::main]
async fn main() -> Result<()> {
    let tick = get_tick_info().await?;
    println!("tick-info: {:?}", tick.tick_info);

    // Replace with a real identity if needed.
    let identity = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAFXIB";
    let balance = get_balance(identity).await?;
    println!("balance: {:?}", balance.balance);

    Ok(())
}
