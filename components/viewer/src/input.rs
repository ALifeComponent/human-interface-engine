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
