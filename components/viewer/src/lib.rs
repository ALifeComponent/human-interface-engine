use std::{f32::consts::FRAC_PI_2, ops::Range};

use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};

pub fn run_app() -> anyhow::Result<()> {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<CameraSettings>()
        .add_systems(Startup, (setup, instructions))
        .add_systems(Update, (camera_movement, zoom))
        .run();

    Ok(())
}

#[derive(Debug, Resource)]
struct CameraSettings {
    pub orbit_distance: f32,
    pub pitch_speed: f32,
    // Clamp pitch to this range
    pub pitch_range: Range<f32>,
    pub roll_speed: f32,
    pub yaw_speed: f32,
    pub pan_speed: f32,
    pub zoom_speed: f32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        // Limiting pitch stops some unexpected rotation past 90° up or down.
        let pitch_limit = FRAC_PI_2 - 0.01;
        Self {
            // These values are completely arbitrary, chosen because they seem to produce
            // "sensible" results for this example. Adjust as required.
            orbit_distance: 20.0,
            pitch_speed: 0.003,
            pitch_range: -pitch_limit..pitch_limit,
            roll_speed: 1.0,
            yaw_speed: 0.004,
            pan_speed: 0.05,
            zoom_speed: 1.5,
        }
    }
}

/// Set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Name::new("Camera"),
        Camera3d::default(),
        Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        Name::new("Plane"),
        Mesh3d(meshes.add(Plane3d::default().mesh().size(5.0, 5.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.5, 0.3),
            // Turning off culling keeps the plane visible when viewed from beneath.
            cull_mode: None,
            ..default()
        })),
    ));

    commands.spawn((
        Name::new("Cube"),
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
        Transform::from_xyz(1.5, 0.51, 1.5),
    ));

    commands.spawn((
        Name::new("Light"),
        PointLight::default(),
        Transform::from_xyz(3.0, 8.0, 5.0),
    ));
}

fn instructions(mut commands: Commands) {
    commands.spawn((
        Name::new("Instructions"),
        Text::new(
            "Mouse left drag: Pan\n\
            Mouse move: Orbit\n\
            Mouse wheel: Zoom",
        ),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.),
            left: Val::Px(12.),
            ..default()
        },
    ));
}

fn camera_movement(
    mut camera: Query<&mut Transform, (With<Camera>, Without<CameraSettings>)>,
    mut camera_settings: ResMut<CameraSettings>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = camera.get_single_mut() {
        let mut delta = mouse_motion.delta;
        let mut pan = Vec2::ZERO;

        // Handle pan (left mouse button)
        if mouse_buttons.pressed(MouseButton::Left) {
            pan = Vec2::new(delta.x, delta.y) * camera_settings.pan_speed;
            delta = Vec2::ZERO; // Don't orbit while panning
        }

        // Calculate orbit rotation
        let delta_pitch = delta.y * camera_settings.pitch_speed;
        let delta_yaw = delta.x * camera_settings.yaw_speed;

        // Obtain the existing pitch, yaw, and roll values from the transform.
        let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);

        // Establish the new yaw and pitch, preventing the pitch value from exceeding our limits.
        let pitch = (pitch + delta_pitch).clamp(
            camera_settings.pitch_range.start,
            camera_settings.pitch_range.end,
        );
        let yaw = yaw + delta_yaw;

        // Update camera rotation
        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);

        // Adjust the translation to maintain the correct orientation toward the orbit target.
        let forward = transform.forward();
        let right = transform.right();
        let up = transform.up();

        // Apply pan movement (local X/Y plane)
        let target = Vec3::ZERO;
        let mut new_target = target - forward * camera_settings.orbit_distance;
        new_target += right * pan.x;
        new_target += up * pan.y;

        // Update camera position
        camera_settings.orbit_distance = (new_target - target).length();
        transform.translation = new_target;
    }
}

fn zoom(
    mut camera: Query<&mut Transform, (With<Camera>, Without<CameraSettings>)>,
    mut camera_settings: ResMut<CameraSettings>,
    mut mouse_wheel: EventReader<MouseWheel>,
    time: Res<Time>,
) {
    let mut scroll = 0.0;
    for ev in mouse_wheel.read() {
        match ev.unit {
            MouseScrollUnit::Line => scroll += ev.y,
            MouseScrollUnit::Pixel => scroll += ev.y / 100.0,
        }
    }

    if scroll != 0.0 && !camera.is_empty() {
        let mut transform = camera.single_mut();
        let forward = transform.forward();
        let target = Vec3::ZERO;

        // Adjust camera distance with zoom speed
        let new_distance = (camera_settings.orbit_distance - scroll * camera_settings.zoom_speed * time.delta_seconds())
            .clamp(1.0, 100.0);

        // Update camera position and settings
        transform.translation = target - forward * new_distance;
        camera_settings.orbit_distance = new_distance;
    }
}
