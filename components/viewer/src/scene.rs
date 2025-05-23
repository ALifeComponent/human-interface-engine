use crate::input::{ToggleAction, ToggleButton};
use bevy::prelude::*;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Startup, instructions)
            .add_systems(Startup, setup_ui);
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Name::new("Light"),
        DirectionalLight {
            color: Color::WHITE,
            illuminance: 10000.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(0., 0., -5.0)
            .with_rotation(Quat::from_rotation_x(-std::f32::consts::PI / 2.)),
    ));
}

pub fn instructions(mut commands: Commands) {
    commands.spawn((
        Name::new("Instructions"),
        Text::new(
            "Left click + drag: rotate\n\
            Scroll: zoom\n\
            [W][A][S][D]: move\n\
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
