use bevy::prelude::*;
use std::fmt::Display;
use uuid::Uuid;

// 線形補間速度を外部から指定する Resource
#[derive(Resource)]
pub struct SmoothMovementSettings {
    pub speed: f32,
}

impl Default for SmoothMovementSettings {
    fn default() -> Self {
        Self { speed: 10.0 }
    }
}

// 各オブジェクトの現在ターゲット位置を保持するコンポーネント
#[derive(Component)]
pub struct TargetPosition(pub Vec3);

pub struct ObjectRequestPlugin;

impl Plugin for ObjectRequestPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SmoothMovementSettings>()
            .add_event::<SpawnObjectRequest>()
            .add_systems(Update, SpawnObjectRequest::event_handler)
            .add_event::<SetObjectPositionRequest>()
            .add_systems(Update, SetObjectPositionRequest::event_handler)
            .add_systems(Update, smooth_movement_system);
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
        mut query: Query<(&ObjectId, &mut TargetPosition)>,
    ) {
        for event in event_reader.read() {
            for (object_id, mut target_pos) in query.iter_mut() {
                if *object_id == event.object_id {
                    info!(
                        "Updating target position of object {} to {:?}",
                        object_id, event.position
                    );
                    target_pos.0 = event.position;
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
            let props = &event.object_properties;
            let pos = event.position;
            match props.shape {
                ObjectShape::Cube => {
                    info!("Spawning cube with size: {}", props.size);
                    commands.spawn((
                        event.object_id.clone(),
                        Name::new(event.object_id.to_string()),
                        Mesh3d(meshes.add(Cuboid::from_size(Vec3::splat(props.size)))),
                        MeshMaterial3d(materials.add(StandardMaterial {
                            base_color: props.color,
                            ..default()
                        })),
                        Transform::from_translation(pos),
                        TargetPosition(pos),
                    ));
                }
                ObjectShape::Sphere => {
                    info!("Spawning sphere with size: {}", props.size);
                    commands.spawn((
                        event.object_id.clone(),
                        Name::new(event.object_id.to_string()),
                        Mesh3d(meshes.add(Sphere::new(props.size))),
                        MeshMaterial3d(materials.add(StandardMaterial {
                            base_color: props.color,
                            ..default()
                        })),
                        Transform::from_translation(pos),
                        TargetPosition(pos),
                    ));
                }
            }
        }
    }
}

// 滑らか移動用システム：毎フレーム、Transform を TargetPosition に向かって線形補間する
fn smooth_movement_system(
    time: Res<Time>,
    settings: Res<SmoothMovementSettings>,
    mut query: Query<(&mut Transform, &TargetPosition)>,
) {
    let alpha = (time.delta_seconds() * settings.speed).clamp(0.0, 1.0);
    for (mut transform, target) in query.iter_mut() {
        transform.translation = transform.translation.lerp(target.0, alpha);
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
