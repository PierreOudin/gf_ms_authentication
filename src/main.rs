pub mod services;
pub mod server_builder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    server_builder::build_server(addr).await?;
    Ok(())
}