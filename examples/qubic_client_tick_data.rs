use anyhow::Result;
use scapi::QubicClient;

#[tokio::main]
async fn main() -> Result<()> {
    let tick = 43438515;

    let data = QubicClient::new().archive().get_tick_data(tick).await?;
    println!("tick {} data: {:#?}", tick, data);

    Ok(())
}
