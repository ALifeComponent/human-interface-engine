use crate::camera::CameraSettings;
use bevy::prelude::*;

/// Bevy plugin that registers input systems for camera inversion toggles.
pub struct InputPlugin;

impl Plugin for InputPlugin {
    /// Registers the system for toggling camera inversion based on input.
    fn build(&self, app: &mut App) {
        app.add_systems(Update, toggle_input_system);
    }
}

#[derive(Component)]
pub struct ToggleButton {
    pub action: ToggleAction,
}

pub enum ToggleAction {
    InvertPitch,
    InvertYaw,
}

/// Toggles camera pitch/yaw inversion on key press and updates UI text.
pub fn toggle_input_system(
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
