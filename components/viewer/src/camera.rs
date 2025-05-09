use std::{f32::consts::FRAC_PI_2, ops::Range};
use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};

#[derive(Debug, Resource)]
pub struct CameraSettings {
    pub orbit_distance: f32,
    pub min_orbit_distance: f32,
    pub max_orbit_distance: f32,
    pub pitch_speed: f32,
    pub pitch_range: Range<f32>,
    pub yaw_speed: f32,
    pub invert_pitch: bool,
    pub invert_yaw: bool,
}

impl Default for CameraSettings {
    fn default() -> Self {
        let pitch_limit = FRAC_PI_2 - 0.01;
        Self {
            orbit_distance: 20.0,
            min_orbit_distance: 1.0,
            max_orbit_distance: 100.0,
            pitch_speed: 0.003,
            pitch_range: -pitch_limit..pitch_limit,
            yaw_speed: 0.004,
            invert_pitch: false,
            invert_yaw: false,
        }
    }
}

pub fn orbit(
    mut camera: Single<&mut Transform, With<Camera>>,
    mut camera_settings: ResMut<CameraSettings>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
) {
    let delta = mouse_motion.delta;
    
    if mouse_buttons.pressed(MouseButton::Left) {
        let mut delta_pitch = delta.y * camera_settings.pitch_speed;
        let mut delta_yaw = delta.x * camera_settings.yaw_speed;

        // 反転設定を適用
        if camera_settings.invert_pitch {
            delta_pitch *= -1.0;
        }
        if camera_settings.invert_yaw {
            delta_yaw *= -1.0;
        }

        let (yaw, pitch, _) = camera.rotation.to_euler(EulerRot::YXZ);
        
        let new_pitch = (pitch + delta_pitch)
            .clamp(camera_settings.pitch_range.start, camera_settings.pitch_range.end);
        let new_yaw = yaw + delta_yaw;

        camera.rotation = Quat::from_euler(EulerRot::YXZ, new_yaw, new_pitch, 0.0);
    }

    let target = Vec3::ZERO;
    camera.translation = target - camera.forward() * camera_settings.orbit_distance;
}
