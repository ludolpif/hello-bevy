use bevy::prelude::*;
use bevy::log::LogPlugin;
use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};
use bevy_dev_console::prelude::*;

pub struct DevModePlugin;

impl Plugin for DevModePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
                // Add the log plugin with the custom log layer
                DefaultPlugins.set(LogPlugin {
                    custom_layer: custom_log_layer,
                    ..default()
                }),
                // Add the dev console plugin itself.
                DevConsolePlugin,
                // Add the FpsOverlayPlugin with it's config
                FpsOverlayPlugin { config:
                    FpsOverlayConfig {
                        text_config: TextStyle {
                            font_size: 32.0,
                            color: Color::srgb(0.0, 1.0, 0.0),
                            font: default(),
                        }
                    }
                },
        )) ;
    }
}
