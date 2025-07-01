use bevy::prelude::*;

// note: Debug was added to make info!("{:?}", csc) work
// see: https://doc.rust-lang.org/std/fmt/#formatting-traits
#[derive(Debug,Component, Reflect, Default)]
#[reflect(Component)]
pub struct ColorSourceComponent {
    pub native_size: Vec2,
    pub transform: Transform,
    pub color: Color,
    /* A `Duration` field that should never be serialized to the scene file, so we skip it.
    #[reflect(skip_serializing)]
    pub _time_since_startup: Duration,
    */
}

pub struct ColorSourcePlugin;

impl Plugin for ColorSourcePlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<ColorSourceComponent>()
            .add_systems(Startup, Self::setup);
    }
}

impl ColorSourcePlugin {
    fn setup() {
        info!("registered ColorSourcePlugin");
    }
}


