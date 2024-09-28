use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};
use bevy::window::{PresentMode,WindowMode,WindowResolution};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct DevModePlugin;

impl Plugin for DevModePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                    // Add the defaults plugins with no framerate limiting
                    DefaultPlugins.set(WindowPlugin {
                        primary_window: Some(Window {
                            present_mode: PresentMode::AutoNoVsync,
                            mode: WindowMode::BorderlessFullscreen,
                            resolution: WindowResolution::new(1920.0, 1080.0)
                                .with_scale_factor_override(1.0),
                                ..default()
                        }),
                        ..default()
                    }),
                    // Add the bevy world inspector default UI (could be closed with a key)
                    WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
                    // Add the FpsOverlayPlugin with it's config
                    FpsOverlayPlugin { config:
                        FpsOverlayConfig {
                            text_config: TextStyle {
                                font_size: 16.0,
                                color: Color::srgb(0.0, 1.0, 0.0),
                                font: default(),
                            }
                        }
                    },
            ))
            .add_systems(Startup, (
                    Self::hello_world,
            ))
            .add_systems(Update, (
                    Self::add_name_to_fpsoverlay_for_worldinspector,
            ));
    }
}

impl DevModePlugin {
    fn hello_world() {
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
