use crate::input::{ToggleAction, ToggleButton};
use bevy::prelude::*;

pub fn setup(
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

pub fn instructions(mut commands: Commands) {
    commands.spawn((
        Name::new("Instructions"),
        Text::new(
            "Left click + drag: rotate\n\
            Scroll: zoom\n\
            [P] Toggle pitch inversion\n\
            [Y] Toggle yaw inversion",
        ),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.),
            left: Val::Px(12.),
            ..default()
        },
    ));
}

pub fn setup_ui(mut commands: Commands) {
    // UIの親ノード
    commands
        .spawn((
            Name::new("UI Panel"),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(12.),
                right: Val::Px(12.),
                width: Val::Px(200.),
                height: Val::Px(100.),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Start,
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.7)),
        ))
        .with_children(|parent| {
            // ピッチ反転トグル
            parent.spawn((
                Name::new("Invert Pitch Toggle"),
                Text::new("Invert Pitch: Off"),
                ToggleButton {
                    action: ToggleAction::InvertPitch,
                },
            ));

            // ヨー反転トグル
            parent.spawn((
                Name::new("Invert Yaw Toggle"),
                Text::new("Invert Yaw: Off"),
                ToggleButton {
                    action: ToggleAction::InvertYaw,
                },
            ));
        });
}
