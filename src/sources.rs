use bevy::prelude::*;

// note: Debug was added to make info!("{:?}", csc) work
// see: https://doc.rust-lang.org/std/fmt/#formatting-traits
#[derive(Debug,Component, Reflect, Default)]
#[reflect(Component)]
pub struct ColorSourceComponent {
    pub native_size: Vec2,
    pub transform: Transform,
    pub color: Color,
}

pub struct ColorSourcePlugin;

impl Plugin for ColorSourcePlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<ColorSourceComponent>()
            .add_systems(Startup, Self::hello_world);
    }
}

impl ColorSourcePlugin {
    fn hello_world() {
        info!("registered ColorSource plugin");
    }
}


