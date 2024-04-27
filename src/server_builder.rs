
use std::net::SocketAddr;
use crate::services::authentication::{authentication::authenticated_service_server::AuthenticatedServiceServer, MyAuthentication};
use sea_orm::DatabaseConnection;
use tonic::transport::Server;


pub async fn build_server(addr: SocketAddr, db: DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    let authentication = MyAuthentication{db:db};

    println!("Connecting to grpc server");

    Server::builder()
        .add_service(AuthenticatedServiceServer::new(authentication))
        .serve(addr).await?;

    Ok(())
}