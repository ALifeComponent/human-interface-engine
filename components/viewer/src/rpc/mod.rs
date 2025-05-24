use bevy::prelude::*;
use serve::{GrpcServer, spawn_grpc_request_system};
mod proto;
mod serve;
mod service;

/// Bevy plugin that initializes and runs the gRPC server for remote object management.
pub struct RpcPlugin;

impl Plugin for RpcPlugin {
    /// Registers the GrpcServer resource and schedules the system to spawn the gRPC server.
    fn build(&self, app: &mut App) {
        app.insert_resource(GrpcServer::default())
            .add_systems(Startup, spawn_grpc_request_system);
    }
}
