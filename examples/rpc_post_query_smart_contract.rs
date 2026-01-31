use anyhow::Result;
use scapi::rpc::post::query_smart_contract_with_meta;

#[tokio::main]
async fn main() -> Result<()> {
    // Example payload. Replace contract_index/input_type/data with real values.
    let contract_index = 0u32;
    let input_type = 0u32;
    let request_bytes: Vec<u8> = Vec::new();

    let response =
        query_smart_contract_with_meta(contract_index, input_type, &request_bytes).await?;
    println!("response bytes: {}", response.len());
    Ok(())
}
