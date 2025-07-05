use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

use crate::AppSettings;

// From https://docs.rs/bevy_enhanced_input/latest/bevy_enhanced_input/
// Input manager for Bevy, inspired by Unreal Engine Enhanced Input
pub struct UserInputPlugin;

#[derive(InputContext)]
struct General;

impl Plugin for UserInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EnhancedInputPlugin)
            .add_input_context::<General>()
            .add_systems(Startup, Self::setup)
            .add_observer(Self::bind_actions);
    }
}

impl UserInputPlugin {
    fn setup(mut commands: Commands) {
        info!("registered DiagnosticsPlugin");
        commands.spawn(Actions::<General>::default());
    }

    /// Setups bindings for [`General`] context from application settings.
    fn bind_actions(
        trigger: Trigger<Bind<General>>,
        settings: Res<AppSettings>,
        mut actions: Query<&mut Actions<General>>,
    ) {
        let mut actions = actions.get_mut(trigger.target()).unwrap();
        actions.bind::<Diag>().to(settings.keyboard.diag);
        actions.bind::<DumpScene>().to(settings.keyboard.dump_scene);
    }
}
pub struct KeyboardSettings {
    diag: KeyCode,
    dump_scene: KeyCode,
}
impl KeyboardSettings {
    pub fn default() -> KeyboardSettings {
        KeyboardSettings {
            diag: KeyCode::F8,
            dump_scene: KeyCode::F9,
        }
    }
}

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
pub struct Diag;

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
pub struct DumpScene;
