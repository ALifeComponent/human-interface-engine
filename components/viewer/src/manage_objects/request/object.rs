use std::fmt::Display;

// ■ 追加：滑らか移動を有効化するフラグ（定数で切り替え）
const ENABLE_SMOOTH_MOVEMENT: bool = true;

use bevy::prelude::{Time, *};
use uuid::Uuid;

pub struct ObjectRequestPlugin;

impl Plugin for ObjectRequestPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SetObjectPositionRequest>()
            .add_systems(Update, SetObjectPositionRequest::event_handler)
            .add_event::<SpawnObjectRequest>()
            .add_systems(Update, SpawnObjectRequest::event_handler);
    }
}

#[derive(Debug)]
pub enum ObjectRequest {
    SetPosition(SetObjectPositionRequest),
    Spawn(SpawnObjectRequest),
}

#[derive(Debug, Clone, Event)]
pub struct SetObjectPositionRequest {
    pub object_id: ObjectId,
    pub position: Vec3,
}

impl SetObjectPositionRequest {
    pub fn event_handler(
        mut event_reader: EventReader<Self>,
        mut query: Query<(&ObjectId, &mut Transform)>,
        time: Res<Time>,
    ) {
        for event in event_reader.read() {
            for (object_id, mut transform) in query.iter_mut() {
                if *object_id == event.object_id {
                    info!(
                        "Setting position of object {} to {:?}",
                        object_id, event.position
                    );
                    if ENABLE_SMOOTH_MOVEMENT {
                        // 線形補間で滑らかに移動
                        let alpha = (time.delta_secs() * 5.0).clamp(0.0, 1.0);
                        transform.translation = transform.translation.lerp(event.position, alpha);
                    } else {
                        // 非スムーズモードでは直接位置設定
                        transform.translation = event.position;
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Event)]
pub struct SpawnObjectRequest {
    pub object_id: ObjectId,
    pub object_properties: ObjectProperties,
    pub position: Vec3,
}

impl SpawnObjectRequest {
    pub fn event_handler(
        mut event_reader: EventReader<Self>,
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        for event in event_reader.read() {
            let object_properties = &event.object_properties;

            // Spawn the object based on the event properties
            match object_properties.shape {
                ObjectShape::Cube => {
                    info!("Spawning cube with size: {}", object_properties.size);
                    commands.spawn((
                        event.object_id.clone(),
                        Name::new(event.object_id.to_string()),
                        Mesh3d(meshes.add(Cuboid::from_size(Vec3::splat(object_properties.size)))),
                        MeshMaterial3d(materials.add(StandardMaterial {
                            base_color: event.object_properties.color,
                            ..default()
                        })),
                        Transform::from_translation(event.position),
                    ));
                }
                ObjectShape::Sphere => {
                    info!("Spawning sphere with size: {}", object_properties.size);
                    commands.spawn((
                        event.object_id.clone(),
                        Name::new(event.object_id.to_string()),
                        Mesh3d(meshes.add(Sphere::new(object_properties.size))),
                        MeshMaterial3d(materials.add(StandardMaterial {
                            base_color: event.object_properties.color,
                            ..default()
                        })),
                        Transform::from_translation(event.position),
                    ));
                }
            }
        }
    }
}

#[derive(Debug, Component, Clone)]
pub enum ObjectShape {
    Cube,
    Sphere,
}

#[derive(Debug, Component, Clone)]
pub struct ObjectProperties {
    pub color: Color,
    pub size: f32,
    pub shape: ObjectShape,
}

#[derive(Debug, Component, Clone, Eq, PartialEq)]
pub struct ObjectId {
    pub uuid: Uuid,
}

impl Display for ObjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ObjectId({})", self.uuid)
    }
}
