use scapi::{RequestDataBuilder, ResponseDecoder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vec = RequestDataBuilder::new()
        .set_contract_index(16)
        .set_input_type(6)
        .send()
        .await?;

    println!(
        "{}",
        ResponseDecoder::new(&vec).u8("currentState")?.to_value()
    );

    Ok(())
}
