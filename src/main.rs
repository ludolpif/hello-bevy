use bevy::prelude::*;
use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};

mod playground;

fn main() {
    let fps_config = FpsOverlayConfig {
        text_config: TextStyle {
            font_size: 32.0,
            color: Color::srgb(0.0, 1.0, 0.0),
            font: default(),
        },
    };

    App::new()
        .add_plugins((
                DefaultPlugins,
                crate::playground::HelloPlugin,
                FpsOverlayPlugin {config: fps_config }
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // We need to spawn a camera (2d or 3d) to see the fps overlay
    commands.spawn(Camera2dBundle::default());
}
