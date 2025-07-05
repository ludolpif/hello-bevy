use bevy::prelude::*;
use bevy_enhanced_input::prelude::Started;

use crate::sources::ColorSourceComponent;

pub struct ScenePersistancePlugin;

impl Plugin for ScenePersistancePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (Self::setup, Self::load_scene_system))
            .add_systems(Update, Self::log_system)
            .add_observer(Self::dump_scene);
    }
}

// The initial scene file will be loaded below and not change when the scene is saved
const SCENE_FILE_PATH: &str = "scenes/load_scene_example.scn.ron";

impl ScenePersistancePlugin {
    //XXX: don't make too much code here, Bevy will got a .bsn file format !
    // https://github.com/bevyengine/bevy/discussions/9538
    // For now code is borrowed from https://github.com/bevyengine/bevy/blob/main/examples/scene/scene.rs

    fn setup() {
        info!("registered ScenePersistancePlugin");
    }
    /// Loads a scene from an asset file and spawns it in the current world.
    ///
    /// Spawning a `DynamicSceneRoot` creates a new parent entity, which then spawns new
    /// instances of the scene's entities as its children. If you modify the
    /// `SCENE_FILE_PATH` scene file, or if you enable file watching, you can see
    /// changes reflected immediately.
    fn load_scene_system(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.spawn((
            DynamicSceneRoot(asset_server.load(SCENE_FILE_PATH)),
            Name::new(SCENE_FILE_PATH),
        ));
    }

    // This system logs all ColorSourceComponent components in our world. Try making a change to a ColorSourceComponent in
    // load_scene_example.scn. If you enable the `file_watcher` cargo feature you should immediately see
    // the changes appear in the console whenever you make a change.
    fn log_system(query: Query<(Entity, &ColorSourceComponent), Changed<ColorSourceComponent>>) {
        for (entity, csc) in &query {
            // note: needs #[derive(Debug,...) on pub struct ColorSourceComponent
            info!("Entity({}) contains {:?}", entity.index(), csc);
        }
    }

    fn dump_scene(_trigger: Trigger<Started<crate::user_input::DumpScene>>, world: &mut World) {
        // https://docs.rs/bevy/latest/bevy/scene/struct.DynamicSceneBuilder.html#method.extract_entities
        let mut query = world.query_filtered::<Entity, With<ColorSourceComponent>>();

        let scene = DynamicSceneBuilder::from_world(&world)
            .extract_entities(query.iter(&world))
            .build();
        // Scenes can be serialized like this:
        let type_registry = world.resource::<AppTypeRegistry>();
        let type_registry = type_registry.read();
        let serialized_scene = scene.serialize(&type_registry).unwrap();

        // Showing the scene in the console
        info!("{}", serialized_scene);
    }
}
