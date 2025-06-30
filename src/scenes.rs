use bevy::prelude::*;

use crate::sources::ColorSourceComponent;

pub struct ScenePersistancePlugin;

impl Plugin for ScenePersistancePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (
                    Self::hello_world,
                    Self::load_scene_system,
            ))
            .add_systems(Update, Self::log_system)
            ;
    }
}

// The initial scene file will be loaded below and not change when the scene is saved
const SCENE_FILE_PATH: &str = "scenes/load_scene_example.scn.ron";

impl ScenePersistancePlugin {
    //XXX: don't make too much code here, Bevy will got a .bsn file format !
    // https://github.com/bevyengine/bevy/discussions/9538
    // For now code is borrowed from https://github.com/bevyengine/bevy/blob/main/examples/scene/scene.rs

    fn hello_world() {
        info!("registered ScenePersistancePlugin");
    }
    /// Loads a scene from an asset file and spawns it in the current world.
    ///
    /// Spawning a `DynamicSceneRoot` creates a new parent entity, which then spawns new
    /// instances of the scene's entities as its children. If you modify the
    /// `SCENE_FILE_PATH` scene file, or if you enable file watching, you can see
    /// changes reflected immediately.
    fn load_scene_system(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.spawn(DynamicSceneRoot(asset_server.load(SCENE_FILE_PATH)));
    }

    // This system logs all ColorSourceComponent components in our world. Try making a change to a ColorSourceComponent in
    // load_scene_example.scn. If you enable the `file_watcher` cargo feature you should immediately see
    // the changes appear in the console whenever you make a change.
    fn log_system(
        query: Query<(Entity, &ColorSourceComponent), Changed<ColorSourceComponent>>
    ) {
        for (entity, csc) in &query {
            // note: needs #[derive(Debug,...) on pub struct ColorSourceComponent
            info!("Entity({}) contains {:?}", entity.index(), csc);
        }
    }
}
