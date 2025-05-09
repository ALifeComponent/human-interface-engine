use std::{f32::consts::FRAC_PI_2, ops::Range};

use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};

pub fn run_app() -> anyhow::Result<()> {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<CameraSettings>()
        .add_systems(Startup, (setup, instructions))
        .add_systems(Update, orbit)
        .run();

    Ok(())
}

#[derive(Debug, Resource)]
struct CameraSettings {
    pub orbit_distance: f32,
    pub min_orbit_distance: f32,
    pub max_orbit_distance: f32,
    pub pitch_speed: f32,
    // Clamp pitch to this range
    pub pitch_range: Range<f32>,
    pub yaw_speed: f32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        // Limiting pitch stops some unexpected rotation past 90° up or down.
        let pitch_limit = FRAC_PI_2 - 0.01;
        Self {
            // These values are completely arbitrary, chosen because they seem to produce
            // "sensible" results for this example. Adjust as required.
            orbit_distance: 20.0,
            min_orbit_distance: 1.0,
            max_orbit_distance: 100.0,
            pitch_speed: 0.003,
            pitch_range: -pitch_limit..pitch_limit,
            yaw_speed: 0.004,
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
            "Left click + drag: rotate\n\
            Scroll: zoom",
        ),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.),
            left: Val::Px(12.),
            ..default()
        },
    ));
}

fn orbit(
    mut camera: Single<&mut Transform, With<Camera>>,
    mut camera_settings: ResMut<CameraSettings>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
) {
    let delta = mouse_motion.delta;
    let scroll = mouse_motion.scroll_delta.y;

    // Handle zoom with scroll wheel
    if scroll != 0.0 {
        let zoom_speed = 1.0;
        let delta_distance = -scroll * zoom_speed;
        camera_settings.orbit_distance = (camera_settings.orbit_distance + delta_distance)
            .clamp(camera_settings.min_orbit_distance, camera_settings.max_orbit_distance);
    }

    // Handle rotation with left mouse button drag
    if mouse_buttons.pressed(MouseButton::Left) {
        let delta_pitch = delta.y * camera_settings.pitch_speed;
        let delta_yaw = delta.x * camera_settings.yaw_speed;

        // Obtain the existing pitch and yaw values from the transform.
        let (yaw, pitch, _) = camera.rotation.to_euler(EulerRot::YXZ);

        // Calculate new pitch and yaw, applying limits
        let new_pitch = (pitch + delta_pitch)
            .clamp(camera_settings.pitch_range.start, camera_settings.pitch_range.end);
        let new_yaw = yaw + delta_yaw;

        // Update camera rotation
        camera.rotation = Quat::from_euler(EulerRot::YXZ, new_yaw, new_pitch, 0.0);
    }

    // Adjust the translation to maintain the correct orientation toward the orbit target.
    let target = Vec3::ZERO;
    camera.translation = target - camera.forward() * camera_settings.orbit_distance;
}
