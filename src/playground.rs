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
        trace!("tracing");
        debug!("solving issues...");
        info!("hello :)");
        warn!("spooky warning");
        error!("scary error");
    }
}

