mod camera;
mod input;
mod scene;

use bevy::prelude::*;
use camera::{CameraSettings, orbit};
use input::handle_zoom;
use scene::{setup, instructions};

pub fn run_app() -> anyhow::Result<()> {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<CameraSettings>()
        .add_systems(Startup, (setup, instructions))
        .add_systems(Update, (orbit, handle_zoom))
        .run();

    Ok(())
}
