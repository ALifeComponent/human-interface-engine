use crate::camera::CameraSettings;
use bevy::{input::mouse::MouseWheel, prelude::*};

pub fn handle_zoom(
    mut camera_settings: ResMut<CameraSettings>,
    mut scroll_events: EventReader<MouseWheel>,
) {
    let mut scroll = 0.0;

    for event in scroll_events.read() {
        scroll += event.y;
    }

    if scroll != 0.0 {
        let zoom_speed = 1.0;
        let delta_distance = -scroll * zoom_speed;
        camera_settings.orbit_distance = (camera_settings.orbit_distance + delta_distance).clamp(
            camera_settings.min_orbit_distance,
            camera_settings.max_orbit_distance,
        );
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
