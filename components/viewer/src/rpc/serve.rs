use crate::rpc::proto::generated::ObjectId;

use super::proto::generated::manage_object_service_server::{
    ManageObjectService, ManageObjectServiceServer,
};
use super::proto::generated::{
    SetObjectPositionRequest, SetObjectPositionResponse, SpawnObjectRequest, SpawnObjectResponse,
};

use bevy::prelude::*;

use std::net::SocketAddr;
use std::thread;
use tokio::runtime::Runtime;
use tonic::{Response, transport::Server};

#[derive(Default)]
pub struct ManageObjectServiceImpl;

#[tonic::async_trait]
impl ManageObjectService for ManageObjectServiceImpl {
    #[doc = " Sets the position of the object."]
    async fn set_object_position(
        &self,
        request: tonic::Request<SetObjectPositionRequest>,
    ) -> std::result::Result<tonic::Response<SetObjectPositionResponse>, tonic::Status> {
        let request = request.into_inner();
        let SetObjectPositionRequest {
            object_id,
            position,
        } = request;

        info!(
            "Received request to set position of object {:?} to {:?}",
            object_id, position
        );

        return Ok(Response::new(SetObjectPositionResponse { success: true }));
    }

    #[doc = " Spawns a new object in the scene."]
    async fn spawn_object(
        &self,
        request: tonic::Request<SpawnObjectRequest>,
    ) -> std::result::Result<tonic::Response<SpawnObjectResponse>, tonic::Status> {
        let request = request.into_inner();
        let SpawnObjectRequest {
            object_type,
            position,
        } = request;

        info!(
            "Received request to spawn object {:?} at position {:?}",
            object_type, position
        );

        let object_id = ObjectId { id: 0 };

        return Ok(Response::new(SpawnObjectResponse {
            object_id: Some(object_id),
        }));
    }
}

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
