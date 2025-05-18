use bevy::prelude::*;
use serve::{GrpcServer, spawn_grpc_request_system};
mod proto;
mod serve;
mod service;

pub struct RpcPlugin;

impl Plugin for RpcPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GrpcServer::default())
            .add_systems(Startup, spawn_grpc_request_system);
    }
}
