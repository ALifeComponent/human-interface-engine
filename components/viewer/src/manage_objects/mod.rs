pub mod global;
pub mod request;

use bevy::prelude::*;

pub struct ManageObejctsPlugin;

impl Plugin for ManageObejctsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(request::InternalRequestCursor::new())
            .add_plugins(request::InternalRequestPlugin);
    }
}
