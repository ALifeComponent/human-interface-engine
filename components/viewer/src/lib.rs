pub mod camera;
pub mod input;
pub mod manage_objects;
pub mod scene;
pub mod types;

use bevy::prelude::*;

pub struct ViewerPlugin;

impl Plugin for ViewerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .add_plugins(manage_objects::ManageObjectsPlugin)
            .add_plugins(camera::CameraPlugin)
            .add_plugins(input::InputPlugin)
            .add_plugins(scene::ScenePlugin);
    }
}
