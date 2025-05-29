use std::net::SocketAddr;

use bevy::prelude::*;
use serve::spawn_grpc_request_system;
mod serve;
mod service;

#[derive(Debug, Resource)]
pub struct GrpcServer {
    pub addr: std::net::SocketAddr,
}

impl GrpcServer {
    pub fn new(addr: SocketAddr) -> Self {
        Self { addr }
    }
}

/// Bevy plugin that initializes and runs the gRPC server for remote object management.
pub struct RpcPlugin;

impl Plugin for RpcPlugin {
    /// Registers the GrpcServer resource and schedules the system to spawn the gRPC server.
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_grpc_request_system);
    }
}
