use bevy::prelude::*;

#[cfg(feature = "dev_mode")]
mod devmode;

//mod playground;
mod sources;
mod scenes;

fn main() {
    let mut app = App::new();

    #[cfg(feature = "dev_mode")]
    app.add_plugins(crate::devmode::DevModePlugin);
    #[cfg(not(feature = "dev_mode"))]
    app.add_plugins(DefaultPlugins);

    app.add_plugins((
                //crate::playground::HelloPlugin,
                crate::sources::ColorSourcePlugin,
                crate::scenes::ScenePersistancePlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());
}
