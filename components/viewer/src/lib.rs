mod camera;
mod input;
mod rpc;
mod scene;

use bevy::prelude::*;
use camera::{CameraSettings, orbit};
use input::{handle_zoom, toggle_input_system};
use scene::{instructions, setup, setup_ui};

pub fn run_app() -> anyhow::Result<()> {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<CameraSettings>()
        .add_systems(Startup, (setup, instructions, setup_ui))
        .add_systems(Update, (orbit, handle_zoom, toggle_input_system))
        .run();

    Ok(())
}
