pub mod object;

use bevy::prelude::*;

use super::global::INTERNAL_REQUEST_LIST;

pub struct InternalRequestPlugin;

impl Plugin for InternalRequestPlugin {
    /// Registers the object request plugin and the request-processing system.
    fn build(&self, app: &mut App) {
        app.add_plugins(object::ObjectRequestPlugin)
            .add_systems(Update, process_requests);
    }
}

/// Processes queued internal requests and emits corresponding spawn/position events.
pub fn process_requests(
    mut request_cursor: ResMut<InternalRequestCursor>,
    mut spawn_request_event: EventWriter<object::SpawnObjectRequest>,
    mut set_position_request_event: EventWriter<object::SetObjectPositionRequest>,
) {
    let reader = INTERNAL_REQUEST_LIST.get_reader();
    if reader.is_err() {
        error!("Failed to get reader for request queue");
        return;
    }
    let reader = reader.unwrap();

    // Process the requests in the queue
    for request in reader[request_cursor.current_position..].iter() {
        match request {
            InternalRequest::ObjectRequest(object_request) => match object_request {
                object::ObjectRequest::Spawn(spawn_request) => {
                    spawn_request_event.write(spawn_request.clone());
                }
                object::ObjectRequest::SetPosition(set_position_request) => {
                    set_position_request_event.write(set_position_request.clone());
                }
            },
        }

        request_cursor.increment();
    }
}

#[derive(Debug)]
pub enum InternalRequest {
    ObjectRequest(object::ObjectRequest),
}

#[derive(Debug, Resource)]
pub struct InternalRequestCursor {
    pub current_position: usize,
}

impl InternalRequestCursor {
    pub fn new() -> Self {
        InternalRequestCursor {
            current_position: 0,
        }
    }

    pub fn reset(&mut self) {
        self.current_position = 0;
    }

    pub fn increment(&mut self) {
        self.current_position += 1;
    }

    pub fn set(&mut self, position: usize) {
        self.current_position = position;
    }
}
