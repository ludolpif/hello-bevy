use bevy::prelude::*;

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, HelloPlugin::hello_world);
    }
}

impl HelloPlugin {
    fn hello_world() {
        println!("Hello, world!");
    }
}

