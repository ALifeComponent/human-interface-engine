mod camera;
mod input;
mod manage_objects;
mod rpc;
mod scene;
mod types;

use bevy::prelude::*;

pub fn run_app() -> anyhow::Result<()> {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(rpc::RpcPlugin)
        .add_plugins(manage_objects::ManageObjectsPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(input::InputPlugin)
        .add_plugins(scene::ScenePlugin)
        .run();

    Ok(())
}
