use bevy::prelude::*;

use super::{ObjectShape, SpawnRequestPosition, global::SPAWN_OBJECT_REQUEST_LIST};

pub fn process_spawn_requests(
    mut commands: Commands,
    mut spawn_request_position: ResMut<SpawnRequestPosition>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let spawn_request = &SPAWN_OBJECT_REQUEST_LIST;

    {
        let reader = spawn_request.queue.get_reader();

        if reader.is_err() {
            error!("Failed to get reader for spawn request queue");
            return;
        }
        let reader = reader.unwrap();

        for request in reader[spawn_request_position.current_position..].iter() {
            // Spawn the object based on the request properties
            match request.object_properties.shape {
                ObjectShape::Cube => {
                    commands.spawn((
                        request.object_id.clone(),
                        Name::new(request.object_id.to_string()),
                        Mesh3d(meshes.add(Cuboid::default())),
                        MeshMaterial3d(materials.add(StandardMaterial {
                            base_color: request.object_properties.color,
                            ..default()
                        })),
                        Transform::from_translation(request.position),
                    ));
                }
                ObjectShape::Sphere => {
                    commands.spawn((
                        request.object_id.clone(),
                        Name::new(request.object_id.to_string()),
                        Mesh3d(meshes.add(Sphere::default())),
                        MeshMaterial3d(materials.add(StandardMaterial {
                            base_color: request.object_properties.color,
                            ..default()
                        })),
                        Transform::from_translation(request.position),
                    ));
                }
            }
            // Update the current position
            spawn_request_position.current_position += 1;
        }
    }
}
