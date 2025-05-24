use bevy::prelude::{Time, *};
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

pub struct ObjectRequestPlugin;

impl Plugin for ObjectRequestPlugin {
    fn build(&self, app: &mut App) {
        app
            // 補間速度の初期値を 5.0 に設定
            .init_resource::<SmoothMovementSettings>()
            .add_event::<SetObjectPositionRequest>()
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
        settings: Res<SmoothMovementSettings>,
    ) {
        for event in event_reader.read() {
            for (object_id, mut transform) in query.iter_mut() {
                if *object_id == event.object_id {
                    info!(
                        "Setting position of object {} to {:?}",
                        object_id, event.position
                    );
                    // Resource で指定した速度で線形補間
                    let alpha = (time.delta_secs() * settings.speed).clamp(0.0, 1.0);
                    transform.translation = transform.translation.lerp(event.position, alpha);
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
