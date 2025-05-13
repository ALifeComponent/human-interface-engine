use bevy::prelude::*;
use serve::spawn_grpc_request_system;
mod proto;
mod serve;

pub struct RpcPlugin;

impl Plugin for RpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_grpc_request_system);
    }
}
