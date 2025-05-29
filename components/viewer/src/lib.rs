mod camera;
mod input;
mod manage_objects;
mod rpc;
mod scene;
mod types;

use bevy::prelude::*;

pub struct ViewerPlugin;

impl Plugin for ViewerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .add_plugins(rpc::RpcPlugin)
            .add_plugins(manage_objects::ManageObjectsPlugin)
            .add_plugins(camera::CameraPlugin)
            .add_plugins(input::InputPlugin)
            .add_plugins(scene::ScenePlugin);
    }
}
