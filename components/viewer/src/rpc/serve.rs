use super::proto::generated::manage_object_service_server::ManageObjectServiceServer;

use bevy::prelude::*;

use std::net::SocketAddr;
use std::thread;
use tokio::runtime::Runtime;
use tonic::transport::Server;

use super::service::ManageObjectServiceImpl;

/// Starts the gRPC server on the specified socket address.
pub async fn serve_grpc(addr: std::net::SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    let manage_object_service = ManageObjectServiceImpl::default();

    Server::builder()
        .add_service(ManageObjectServiceServer::new(manage_object_service))
        .serve(addr)
        .await?;
    Ok(())
}

#[derive(Debug, Resource)]
pub struct GrpcServer {
    pub addr: std::net::SocketAddr,
}

impl Default for GrpcServer {
    fn default() -> Self {
        Self {
            addr: SocketAddr::from(([127, 0, 0, 1], 50051)),
        }
    }
}

/// Spawns a thread running the Tokio runtime to serve the gRPC server.
pub fn spawn_grpc_request_system(grpc_server: Res<GrpcServer>) {
    let addr = grpc_server.addr;

    thread::spawn(move || {
        let rt = Runtime::new().expect("Failed to create Tokio runtime");
        rt.block_on(async {
            serve_grpc(addr).await.expect("Failed to start gRPC server");
        });
    });

    info!("gRPC server thread spawned on {}", addr);
}
