pub mod services;
pub mod server_builder;
pub mod db;

use std::time::Duration;

use dotenvy::dotenv;
use sea_orm::{ConnectOptions, Database};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found in .env");

    let mut opt = ConnectOptions::new(db_url);

    opt.max_connections(10)
    .connect_timeout(Duration::from_secs(8))
    .acquire_timeout(Duration::from_secs(8))
    .idle_timeout(Duration::from_secs(8))
    .max_lifetime(Duration::from_secs(8))
    .set_schema_search_path("my_schema");

    let database = Database::connect(opt).await?;

    let _ = db::migration::create_tables(&database).await;

    let addr = "[::1]:50051".parse()?;
    server_builder::build_server(addr).await?;
    Ok(())
}