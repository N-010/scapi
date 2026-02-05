use anyhow::Result;
use scapi::rpc::get::get_tick_data;

#[tokio::main]
async fn main() -> Result<()> {
    let tick = 43438515;

    let data = get_tick_data(tick).await?;
    match data.tick_data {
        Some(tick_data) => println!("tick {} data: {:#?}", tick, tick_data),
        None => println!("tick {} has no archived data yet", tick),
    }

    Ok(())
}
