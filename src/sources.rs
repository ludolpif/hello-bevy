use bevy::prelude::*;

#[derive(Component, Reflect, Default)]
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
            .add_systems(Startup, ColorSourcePlugin::hello_world);
    }
}

impl ColorSourcePlugin {
    fn hello_world() {
        info!("registered ColorSource plugin");
    }
}


