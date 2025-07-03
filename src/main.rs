// https://bevy-cheatbook.github.io/platforms/windows.html#disabling-the-windows-console
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode, WindowResolution};
use bevy::winit::{UpdateMode, WinitSettings};

#[cfg(feature = "dev_mode")]
mod devmode;

//mod playground;
mod components;
mod core;
mod diagnostics;
mod scenes;
mod sources;
mod systemsets;
mod userinput;

const WINDOW_TITLE: &str = "Hello Bevy";

#[derive(Resource)]
struct AppSettings {
    keyboard: KeyboardSettings,
}

struct KeyboardSettings {
    diag: KeyCode,
}

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                present_mode: PresentMode::AutoNoVsync,
                title: WINDOW_TITLE.into(),
                resolution: WindowResolution::new(1920.0, 1080.0).with_scale_factor_override(1.0),
                ..default()
            }),
            ..default()
        }),
        // Limit CPU usage without blocking on VSync
        bevy_framepace::FramepacePlugin,
    ));

    #[cfg(feature = "dev_mode")]
    app.add_plugins(crate::devmode::DevModePlugin);

    app.add_plugins((
        //crate::playground::HelloPlugin,
        crate::core::CoreLogicPlugin,
        crate::diagnostics::DiagnosticsPlugin,
        crate::sources::ColorSourcePlugin,
        crate::scenes::ScenePersistancePlugin,
        crate::userinput::UserInputPlugin,
    ))
    .insert_resource(WinitSettings {
        focused_mode: UpdateMode::Continuous,
        unfocused_mode: UpdateMode::Continuous,
    })
    .insert_resource(AppSettings {
        keyboard: KeyboardSettings { diag: KeyCode::F8 }
    })
    .add_systems(Startup, setup)
    .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
