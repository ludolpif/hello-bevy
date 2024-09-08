use bevy::{prelude::*, tasks::IoTaskPool, utils::Duration};
use std::{fs::File, io::Write};

use crate::sources::ColorSourceComponent;

pub struct ScenePersistancePlugin;

impl Plugin for ScenePersistancePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (
                    Self::hello_world,
                    //Self::save_scene_system,
                    Self::load_scene_system,
            ))
            .add_systems(Update, Self::log_system)
            ;
    }
}

// The initial scene file will be loaded below and not change when the scene is saved
const SCENE_FILE_PATH: &str = "scenes/load_scene_example.scn.ron";

// The new, updated scene data will be saved here so that you can see the changes
const NEW_SCENE_FILE_PATH: &str = "scenes/load_scene_example-new.scn.ron";

impl ScenePersistancePlugin {
    fn hello_world() {
        info!("registered ScenePersistancePlugin");
    }
    fn load_scene_system(mut commands: Commands, asset_server: Res<AssetServer>) {
        // "Spawning" a scene bundle creates a new entity and spawns new instances
        // of the given scene's entities as children of that entity.
        commands.spawn(DynamicSceneBundle {
            // Scenes are loaded just like any other asset.
            scene: asset_server.load(SCENE_FILE_PATH),
            ..default()
        });
    }

    // This system logs all ComponentA components in our world. Try making a change to a ComponentA in
    // load_scene_example.scn. If you enable the `file_watcher` cargo feature you should immediately see
    // the changes appear in the console whenever you make a change.
    fn log_system(
        query: Query<(Entity, &ColorSourceComponent), Changed<ColorSourceComponent>>
    ) {
        for (entity, component_a) in &query {
            info!("  Entity({})", entity.index());
            info!(
                "    ComponentA: {{ w: {} h: {} }}\n",
                component_a.native_size.x, component_a.native_size.y
            );
        }
    }

    fn save_scene_system(world: &mut World) {
        // Scenes can be created from any ECS World.
        // You can either create a new one for the scene or use the current World.
        // For demonstration purposes, we'll create a new one.
        let mut scene_world = World::new();

        // The `TypeRegistry` resource contains information about all registered types (including components).
        // This is used to construct scenes, so we'll want to ensure that our previous type registrations
        // exist in this new scene world as well.
        // To do this, we can simply clone the `AppTypeRegistry` resource.
        let type_registry = world.resource::<AppTypeRegistry>().clone();
        scene_world.insert_resource(type_registry);

        /*
        let mut component_b = ComponentB::from_world(world);
        component_b.value = "hello".to_string();
        scene_world.spawn((
                component_b,
                ComponentA { x: 1.0, y: 2.0 },
                Transform::IDENTITY,
                Name::new("joe"),
        ));
        scene_world.spawn(ComponentA { x: 3.0, y: 4.0 });
        scene_world.insert_resource(ResourceA { score: 1 });
        */
        scene_world.spawn(ColorSourceComponent {
            native_size: Vec2::new(1920.0, 1080.0),
            transform: Transform::IDENTITY,
            color: Color::linear_rgba(1.0,0.0,0.0,0.5)
        } );
        // With our sample world ready to go, we can now create our scene using DynamicScene or DynamicSceneBuilder.
        // For simplicity, we will create our scene using DynamicScene:
        let scene = DynamicScene::from_world(&scene_world);

        // Scenes can be serialized like this:
        let type_registry = world.resource::<AppTypeRegistry>();
        let type_registry = type_registry.read();
        let serialized_scene = scene.serialize(&type_registry).unwrap();

        // Showing the scene in the console
        info!("{}", serialized_scene);

        // Writing the scene to a new file. Using a task to avoid calling the filesystem APIs in a system
        // as they are blocking
        // This can't work in WASM as there is no filesystem access
        #[cfg(not(target_arch = "wasm32"))]
        IoTaskPool::get()
            .spawn(async move {
                // Write the scene RON data to file
                File::create(format!("assets/{NEW_SCENE_FILE_PATH}"))
                    .and_then(|mut file| file.write(serialized_scene.as_bytes()))
                    .expect("Error while writing scene to file");
                })
        .detach();
    }
}
