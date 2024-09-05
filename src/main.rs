use bevy::prelude::*;

#[cfg(feature = "dev_mode")]
use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};

mod playground;

fn main() {
    #[cfg(feature = "dev_mode")]
    let fps_config = FpsOverlayConfig {
        text_config: TextStyle {
            font_size: 32.0,
            color: Color::srgb(0.0, 1.0, 0.0),
            font: default(),
        },
    };

    let mut app = App::new();

    app.add_plugins((
                DefaultPlugins,
                crate::playground::HelloPlugin,
        ))
        .add_systems(Startup, setup);

    #[cfg(feature = "dev_mode")]
    app.add_plugins(
        FpsOverlayPlugin { config: fps_config }
    );

    app.run();
}

fn setup(mut commands: Commands) {
    // We need to spawn a camera (2d or 3d) to see the fps overlay
    commands.spawn(Camera2dBundle::default());
}
