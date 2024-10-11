use bevy::prelude::*;
use bevy::winit::{UpdateMode,WinitSettings};

// https://bevy-cheatbook.github.io/platforms/windows.html#disabling-the-windows-console
#![cfg(not(feature = "dev_mode")), windows_subsystem = "windows")]

#[cfg(feature = "dev_mode")]
mod devmode;

//mod playground;
mod components;
mod core;
mod scenes;
mod sources;
mod systemsets;

fn main() {
    let mut app = App::new();

    #[cfg(feature = "dev_mode")]
    app.add_plugins(crate::devmode::DevModePlugin);
    #[cfg(not(feature = "dev_mode"))]
    app.add_plugins(DefaultPlugins);

    app.add_plugins((
                //crate::playground::HelloPlugin,
                crate::core::CoreLogicPlugin,
                crate::sources::ColorSourcePlugin,
                crate::scenes::ScenePersistancePlugin,
        ))
        .insert_resource(WinitSettings {
            focused_mode: UpdateMode::Continuous,
            unfocused_mode: UpdateMode::Continuous,
        })
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());
}
