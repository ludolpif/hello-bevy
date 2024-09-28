use bevy::prelude::*;
use crate::systemsets::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MyAppState {
    #[default]
    Idling,
    Encoding,
}

pub struct CoreLogicPlugin;

impl Plugin for CoreLogicPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<MyAppState>()
            .configure_sets(Update, (
                MyConfigSystemSet::OfflineEditable
                .run_if(in_state(MyAppState::Idling)),
                MyConfigSystemSet::LiveEditable,
                MyCompositingSystemSet::Previews
                .after(MyConfigSystemSet::LiveEditable),
                MyCompositingSystemSet::Programs
                .run_if(in_state(MyAppState::Encoding))
                .after(MyConfigSystemSet::LiveEditable),
            ))
            .add_systems(Startup, (
                    Self::hello_world,
            ));
    }
}

impl CoreLogicPlugin {
    fn hello_world() {
        info!("registered CoreLogicPlugin");
    }
}
