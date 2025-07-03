use std::time::Duration;

use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

// From: https://bevy.org/examples/dev-tools/fps-overlay/

pub struct DevModePlugin;

const FPS_OVERLAY_UPDATES_PER_SECOND: f64 = 30.0;

impl Plugin for DevModePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                    bevy_mod_debugdump::CommandLineArgs,
                    // Add the bevy world inspector default UI (could be closed with a key)
                    EguiPlugin { enable_multipass_for_primary_context: true },
                    WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
                    // Add the FpsOverlayPlugin with it's config
                    FpsOverlayPlugin { config:
                        FpsOverlayConfig {
                            text_config: TextFont {
                                font_size: 16.0,
                                ..default()
                            },
                            text_color: Color::srgba(1.0, 1.0, 0.0, 0.7),
                            refresh_interval: Duration::from_secs_f64(1.0/FPS_OVERLAY_UPDATES_PER_SECOND),
                            ..default()
                        }
                    },
            ))
            .add_systems(Startup, Self::setup)
            .add_systems(Update, (
                    //FIXME seem evaluated too much
                    Self::add_name_to_fpsoverlay_for_worldinspector,
            ));
    }
}

impl DevModePlugin {
    fn setup() {
        info!("registered DevModePlugin");
    }
    fn add_name_to_fpsoverlay_for_worldinspector(query: Query<Entity, (Without<Name>, With<Children>, With<ZIndex>)>, mut commands: Commands) {
        for entity_id in query.iter() {
            let name = Name::new("FpsOverlay");
            info!("inserting Name {name:?} on {entity_id:?}");
            commands.entity(entity_id).insert(name);
        }
    }
}
