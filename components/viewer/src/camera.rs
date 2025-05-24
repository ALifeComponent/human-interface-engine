use bevy::{
    input::mouse::{AccumulatedMouseMotion, MouseWheel},
    prelude::*,
};
use std::{f32::consts::FRAC_PI_2, ops::Range};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraSettings>()
            .init_resource::<CameraTarget>()
            .add_systems(Startup, setup_camera)
            .add_systems(
                Update,
                (orbit, handle_zoom, handle_movement, handle_drag).chain(), // 競合を避けるため直列実行
            );
    }
}

#[derive(Resource, Debug, Default)]
struct CameraTarget(Vec3);

#[derive(Resource, Debug)]
pub struct CameraSettings {
    pub orbit_distance: f32,
    pub min_orbit_distance: f32,
    pub max_orbit_distance: f32,
    pub pitch_speed: f32,
    pub pitch_range: Range<f32>,
    pub yaw_speed: f32,
    pub invert_pitch: bool,
    pub invert_yaw: bool,
    pub move_speed: f32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        let limit = FRAC_PI_2 - 0.01;
        Self {
            orbit_distance: 20.0,
            min_orbit_distance: 1.0,
            max_orbit_distance: 100.0,
            pitch_speed: -0.003,
            pitch_range: -limit..limit,
            yaw_speed: -0.004,
            invert_pitch: false,
            invert_yaw: false,
            move_speed: 30.0,
        }
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Main Camera"),
        Camera3d::default(),
        Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

/// 回転と距離だけを担当。注視点は `CameraTarget` から取得。
fn orbit(
    mut camera: Single<&mut Transform, With<Camera>>,
    camera_settings: Res<CameraSettings>,
    target: Res<CameraTarget>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
) {
    let delta = mouse_motion.delta;
    if mouse_buttons.pressed(MouseButton::Left) {
        let mut dp = delta.y * camera_settings.pitch_speed;
        let mut dy = delta.x * camera_settings.yaw_speed;
        if camera_settings.invert_pitch {
            dp = -dp;
        }
        if camera_settings.invert_yaw {
            dy = -dy;
        }

        let (yaw, pitch, _) = camera.rotation.to_euler(EulerRot::YXZ);
        let new_pitch = (pitch + dp).clamp(
            camera_settings.pitch_range.start,
            camera_settings.pitch_range.end,
        );
        let new_yaw = yaw + dy;
        camera.rotation = Quat::from_euler(EulerRot::YXZ, new_yaw, new_pitch, 0.0);
    }

    camera.translation = target.0 - camera.forward() * camera_settings.orbit_distance;
}

/// マウスホイールでズーム
fn handle_zoom(mut settings: ResMut<CameraSettings>, mut wheels: EventReader<MouseWheel>) {
    let scroll: f32 = wheels.read().map(|e| e.y).sum();
    if scroll != 0.0 {
        settings.orbit_distance = (settings.orbit_distance - scroll)
            .clamp(settings.min_orbit_distance, settings.max_orbit_distance);
    }
}

/// キーボードで注視点を平行移動
fn handle_movement(
    mut target: ResMut<CameraTarget>,
    camera_settings: Res<CameraSettings>,
    camera: Single<&mut Transform, With<Camera>>, // 向きの計算に使う
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut delta = Vec3::ZERO;

    if keys.pressed(KeyCode::KeyW) {
        delta += *camera.forward();
    }
    if keys.pressed(KeyCode::KeyS) {
        delta -= *camera.forward();
    }
    if keys.pressed(KeyCode::KeyA) {
        delta -= *camera.right();
    }
    if keys.pressed(KeyCode::KeyD) {
        delta += *camera.right();
    }

    if delta != Vec3::ZERO {
        target.0 += delta.normalize() * camera_settings.move_speed * time.delta_secs();
    }
}

/// 中ボタンドラッグで平行移動（パン）
fn handle_drag(
    mut target: ResMut<CameraTarget>,
    camera: Single<&mut Transform, With<Camera>>,
    buttons: Res<ButtonInput<MouseButton>>,
    motion: Res<AccumulatedMouseMotion>,
) {
    if buttons.pressed(MouseButton::Middle) {
        let d = motion.delta;
        // 画面上の移動量をワールド座標に
        target.0 += (-camera.right() * d.x + camera.up() * d.y) * 0.1;
    }
}
