mod camera;
mod input;
mod scene;

use bevy::prelude::*;
use camera::{CameraSettings, orbit};
use input::handle_zoom;
use scene::{ToggleAction, ToggleButton, instructions, setup, setup_ui};

pub fn run_app() -> anyhow::Result<()> {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<CameraSettings>()
        .add_systems(Startup, (setup, instructions, setup_ui))
        .add_systems(Update, (orbit, handle_zoom, toggle_input_system))
        .run();

    Ok(())
}

fn toggle_input_system(
    mut camera_settings: ResMut<CameraSettings>,
    mut query: Query<(&mut Text, &ToggleButton)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    // キーボードショートカットの処理
    if keyboard_input.just_pressed(KeyCode::KeyP) {
        camera_settings.invert_pitch = !camera_settings.invert_pitch;
    }
    if keyboard_input.just_pressed(KeyCode::KeyY) {
        camera_settings.invert_yaw = !camera_settings.invert_yaw;
    }

    // UIテキストの更新
    for (mut text, button) in query.iter_mut() {
        let state = match button.action {
            ToggleAction::InvertPitch => {
                if camera_settings.invert_pitch {
                    "On"
                } else {
                    "Off"
                }
            }
            ToggleAction::InvertYaw => {
                if camera_settings.invert_yaw {
                    "On"
                } else {
                    "Off"
                }
            }
        };

        let label = match button.action {
            ToggleAction::InvertPitch => "Invert Pitch: ",
            ToggleAction::InvertYaw => "Invert Yaw: ",
        };

        text.0 = format!("{}{}", label, state);
    }
}
