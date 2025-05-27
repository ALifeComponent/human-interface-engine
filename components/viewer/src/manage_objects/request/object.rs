use bevy::prelude::*;
use std::fmt::Display;
use uuid::Uuid;

// Resource specifying smooth interpolation speed and enable/disable flag
#[derive(Resource)]
pub struct SmoothMovementSettings {
    /// Interpolation speed (higher value moves faster towards the target)
    pub speed: f32,
    /// Whether interpolation is enabled
    pub enabled: bool,
}

impl Default for SmoothMovementSettings {
    fn default() -> Self {
        Self {
            speed: 10.0,
            enabled: true,
        }
    }
}

/// Component holding the current target position for each object
#[derive(Component)]
pub struct TargetPosition(pub Vec3);

pub struct ObjectRequestPlugin;

impl Plugin for ObjectRequestPlugin {
    /// Initializes smooth movement settings and registers object request systems.
    fn build(&self, app: &mut App) {
        app
            // Initialize resource using Default
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
    /// Handles incoming position events by updating entities’ target positions.
    pub fn event_handler(
        mut event_reader: EventReader<Self>,
        mut query: Query<(&ObjectId, &mut TargetPosition)>,
    ) {
        for event in event_reader.read() {
            for (object_id, mut target_pos) in query.iter_mut() {
                if *object_id == event.object_id {
                    trace!(
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
    /// Handles spawn events by creating new entities with given properties.
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
                    trace!("Spawning cube with size: {}", props.size);
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
                    trace!("Spawning sphere with size: {}", props.size);
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

/// Smoothly interpolates each entity’s transform toward its target position.
fn smooth_movement_system(
    time: Res<Time>,
    settings: Res<SmoothMovementSettings>,
    mut query: Query<(&mut Transform, &TargetPosition)>,
) {
    // Calculate interpolation ratio
    let alpha = (time.delta_secs() * settings.speed).clamp(0.0, 1.0);
    query.par_iter_mut().for_each(|(mut transform, target)| {
        if settings.enabled {
            // With interpolation
            transform.translation = transform.translation.lerp(target.0, alpha);
        } else {
            // Without interpolation
            transform.translation = target.0;
        }
    });
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
