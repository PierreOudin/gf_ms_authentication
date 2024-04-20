
use std::net::SocketAddr;
use crate::services::authentication::{authentication::authenticated_service_server::AuthenticatedServiceServer, MyAuthentication};
use tonic::transport::Server;


pub async fn build_server(addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    let authentication = MyAuthentication::default();

    Server::builder()
        .add_service(AuthenticatedServiceServer::new(authentication))
        .serve(addr).await?;

    Ok(())
}