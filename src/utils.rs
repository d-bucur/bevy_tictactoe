use std::fs::File;
use std::io::Write;

use bevy::{prelude::*, tasks::IoTaskPool};

const SCENE_FILE_PATH: &str = "scenes/saved.ron";
const LOAD_FILE_PATH: &str = "scenes/loaded.ron";

pub fn save_to_scene(world: &World) {
    let type_registry = world.resource::<AppTypeRegistry>();
    let scene = DynamicScene::from_world(&world, type_registry);
    let serialized_scene = scene.serialize_ron(type_registry).unwrap();
    info!("{}", serialized_scene);

    #[cfg(not(target_arch = "wasm32"))]
    IoTaskPool::get()
        .spawn(async move {
            // Write the scene RON data to file
            File::create(format!("assets/{SCENE_FILE_PATH}"))
                .and_then(|mut file| file.write(serialized_scene.as_bytes()))
                .expect("Error while writing scene to file");
        })
        .detach();
}

pub fn load_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    // "Spawning" a scene bundle creates a new entity and spawns new instances
    // of the given scene's entities as children of that entity.
    commands.spawn(DynamicSceneBundle {
        // Scenes are loaded just like any other asset.
        scene: asset_server.load(LOAD_FILE_PATH),
        ..default()
    });
}