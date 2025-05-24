pub mod global;
pub mod request;

use bevy::prelude::*;

/// Bevy plugin that sets up the infrastructure for processing object spawn and movement requests.
pub struct ManageObjectsPlugin;

impl Plugin for ManageObjectsPlugin {
    /// Inserts the InternalRequestCursor resource and adds the InternalRequestPlugin.
    fn build(&self, app: &mut App) {
        app.insert_resource(request::InternalRequestCursor::new())
            .add_plugins(request::InternalRequestPlugin);
    }
}
