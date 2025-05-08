use bevy::prelude::*;

pub fn run_app() -> anyhow::Result<()> {
    App::new()
        // Set the default plugin
        .add_plugins(DefaultPlugins)
        .run();

    Ok(())
}
