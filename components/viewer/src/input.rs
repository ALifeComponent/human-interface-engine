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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_zoom_with_positive_scroll() {
        // テスト用アプリケーションをセットアップ
        let mut app = App::new();

        // 必要なリソースを追加
        let mut settings = CameraSettings::default();
        settings.orbit_distance = 20.0;
        app.insert_resource(settings);

        // マウスホイールイベントを追加
        let mut events = app.world_mut().resource_mut::<Events<MouseWheel>>();
        events.send(MouseWheel {
            unit: MouseScrollDelta::Line,
            y: 1.0,
            ..default()
        });

        // システムを実行
        handle_zoom(
            app.world_mut().resource_mut::<CameraSettings>().clone(),
            app.world_mut().resource_mut::<Events<MouseWheel>>().read(),
        );

        // ズームが正しく更新されたか検証
        let updated_settings = app.world().resource::<CameraSettings>();
        assert!(updated_settings.orbit_distance < 20.0);
    }

    #[test]
    fn test_handle_zoom_with_negative_scroll() {
        // テスト用アプリケーションをセットアップ
        let mut app = App::new();

        // 必要なリソースを追加
        let mut settings = CameraSettings::default();
        settings.orbit_distance = 20.0;
        app.insert_resource(settings);

        // マウスホイールイベントを追加
        let mut events = app.world_mut().resource_mut::<Events<MouseWheel>>();
        events.send(MouseWheel {
            unit: MouseScrollDelta::Line,
            y: -1.0,
            ..default()
        });

        // システムを実行
        handle_zoom(
            app.world_mut().resource_mut::<CameraSettings>().clone(),
            app.world_mut().resource_mut::<Events<MouseWheel>>().read(),
        );

        // ズームが正しく更新されたか検証
        let updated_settings = app.world().resource::<CameraSettings>();
        assert!(updated_settings.orbit_distance > 20.0);
    }

    #[test]
    fn test_handle_zoom_clamping() {
        // テスト用アプリケーションをセットアップ
        let mut app = App::new();

        // 最小値でテスト
        let mut settings = CameraSettings::default();
        settings.orbit_distance = settings.min_orbit_distance;
        app.insert_resource(settings);

        // マウスホイールイベント（縮小しようとする）
        let mut events = app.world_mut().resource_mut::<Events<MouseWheel>>();
        events.send(MouseWheel {
            unit: MouseScrollDelta::Line,
            y: 1.0,
            ..default()
        });

        // システムを実行
        handle_zoom(
            app.world_mut().resource_mut::<CameraSettings>().clone(),
            app.world_mut().resource_mut::<Events<MouseWheel>>().read(),
        );

        // 最小値でクランプされていることを確認
        let updated_settings = app.world().resource::<CameraSettings>();
        assert_eq!(
            updated_settings.orbit_distance,
            updated_settings.min_orbit_distance
        );

        // 最大値でテスト
        let mut settings = CameraSettings::default();
        settings.orbit_distance = settings.max_orbit_distance;
        *app.world_mut().resource_mut::<CameraSettings>() = settings;

        // マウスホイールイベント（拡大しようとする）
        events.send(MouseWheel {
            unit: MouseScrollDelta::Line,
            y: -1.0,
            ..default()
        });

        // システムを実行
        handle_zoom(
            app.world_mut().resource_mut::<CameraSettings>().clone(),
            app.world_mut().resource_mut::<Events<MouseWheel>>().read(),
        );

        // 最大値でクランプされていることを確認
        let updated_settings = app.world().resource::<CameraSettings>();
        assert_eq!(
            updated_settings.orbit_distance,
            updated_settings.max_orbit_distance
        );
    }
}
