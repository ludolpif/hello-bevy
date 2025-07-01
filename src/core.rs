use bevy::prelude::*;
use crate::systemsets::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MyAppState {
    #[default]
    Idling,
    Encoding,
}

pub struct CoreLogicPlugin;

// See https://docs.rs/bevy/latest/bevy/ecs/schedule/trait.SystemSet.html (incomplete in 0.16.1)
// or https://dev-docs.bevy.org/bevy/ecs/schedule/trait.SystemSet.html#adding-systems-to-system-sets
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
            /* Systems may be added here but mostly from other plugins, they can use .in_set()
            .add_systems(Update, (
                Self::system1.in_set(MyConfigSystemSet::OfflineEditable),
                Self::system2.in_set(MyConfigSystemSet::LiveEditable),
                Self::system3.in_set(MyCompositingSystemSet::Previews),
                Self::system4.in_set(MyCompositingSystemSet::Programs),
            ))
            */
            .add_systems(Startup, Self::setup);
    }
}

impl CoreLogicPlugin {
    fn setup() {
        info!("registered CoreLogicPlugin");
    }
    /*
    fn system1() {
        info!("system1")
    }
    fn system2() {
        info!("system2")
    }
    fn system3() {
        info!("system3")
    }
    fn system4() {
        info!("system4")
    }
    */
}
