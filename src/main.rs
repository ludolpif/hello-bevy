use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .run();
}

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

