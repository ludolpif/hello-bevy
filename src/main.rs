use bevy::prelude::*;

#[cfg(feature = "dev_mode")]
mod devmode;

mod playground;

fn main() {
    let mut app = App::new();

    #[cfg(feature = "dev_mode")]
    app.add_plugins(crate::devmode::DevModePlugin);
    #[cfg(not(feature = "dev_mode"))]
    app.add_plugins(DefaultPlugins);

    app.add_plugins((
                crate::playground::HelloPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // We need to spawn a camera (2d or 3d) to see the fps overlay
    commands.spawn(Camera2dBundle::default());
}
