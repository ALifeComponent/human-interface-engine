pub mod global;
mod spawn;

use std::fmt::Display;

use bevy::prelude::*;
use uuid::Uuid;

#[derive(Debug, Resource, Clone)]
pub enum ObjectShape {
    Cube,
    Sphere,
}

#[derive(Debug, Resource, Clone)]
pub struct ObjectProperties {
    pub color: Color,
    pub size: f32,
    pub shape: ObjectShape,
}

#[derive(Debug, Component, Clone)]
pub struct ObjectId {
    pub uuid: Uuid,
}

impl Display for ObjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ObjectId({})", self.uuid)
    }
}

#[derive(Debug, Resource, Clone)]
pub struct SpawnObjectRequest {
    pub object_id: ObjectId,
    pub object_properties: ObjectProperties,
    pub position: Vec3,
}

#[derive(Debug, Resource, Clone)]
pub struct SpawnRequestPosition {
    pub current_position: usize,
}

impl SpawnRequestPosition {
    pub fn new() -> Self {
        SpawnRequestPosition {
            current_position: 0,
        }
    }
}

pub struct ManageObejctsPlugin;

impl Plugin for ManageObejctsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnRequestPosition::new())
            .add_systems(Update, spawn::process_spawn_requests);
    }
}
