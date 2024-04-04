use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!");

    let _response = frontend::run().await;

    Ok(())
}
