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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toggle_input_system() {
        // テスト用アプリケーションをセットアップ
        let mut app = App::new();

        // 必要なリソースを追加
        app.insert_resource(CameraSettings::default());

        // キーボード入力の模擬
        let mut keyboard_input = ButtonInput::<KeyCode>::default();
        keyboard_input.press(KeyCode::KeyP);
        keyboard_input.press(KeyCode::KeyY);
        app.insert_resource(keyboard_input);

        // UI要素を生成
        let entity1 = app
            .world_mut()
            .spawn((
                Text::new("Invert Pitch: Off"),
                ToggleButton {
                    action: ToggleAction::InvertPitch,
                },
            ))
            .id();

        let entity2 = app
            .world_mut()
            .spawn((
                Text::new("Invert Yaw: Off"),
                ToggleButton {
                    action: ToggleAction::InvertYaw,
                },
            ))
            .id();

        // システムを実行
        toggle_input_system(
            app.world_mut().resource_mut::<CameraSettings>().clone(),
            app.world_mut().query::<(&mut Text, &ToggleButton)>(),
            app.world().resource::<ButtonInput<KeyCode>>().clone(),
        );

        // カメラ設定が正しく切り替わったか検証
        let settings = app.world().resource::<CameraSettings>();
        assert!(settings.invert_pitch);
        assert!(settings.invert_yaw);

        // UIテキストが正しく更新されたか検証
        let query = app.world().query::<(&Text, &ToggleButton)>();
        for (text, button) in query.iter(app.world()) {
            match button.action {
                ToggleAction::InvertPitch => assert_eq!(text.0, "Invert Pitch: On"),
                ToggleAction::InvertYaw => assert_eq!(text.0, "Invert Yaw: On"),
            }
        }
    }
}
