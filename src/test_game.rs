use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};

use crate::{palette, AppState};
use rand::Rng;

#[derive(Component)]
struct Ship;

#[derive(Resource, Default)]
struct GameResources {
    mesh: Mesh2dHandle,
    material: Handle<ColorMaterial>
}

pub struct TestGamePlugin;

impl Plugin for TestGamePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(GameResources::default())
            // .add_system_set(SystemSet::on_enter(AppState::Game).with_system(load_scene_test))
            .add_system_set(SystemSet::on_enter(AppState::Game).with_system(setup));
    }
}

fn load_scene_test(commands: Commands, asset_server: Res<AssetServer>) {
    crate::utils::load_scene(commands, asset_server)
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut resources: ResMut<GameResources>
) {
    resources.mesh = meshes.add(shape::Circle::new(30.).into()).into();
    resources.material = materials.add(ColorMaterial::from(palette::SHADE_LIGHT));
    for i in 0..3 {
        spawn_ship(&mut commands, &resources);
    }
}

fn spawn_ship(
    commands: &mut Commands,
    resources: &ResMut<GameResources>,
) {
    commands
        .spawn(MaterialMesh2dBundle {
            transform: Transform::from_xyz(rand::thread_rng().gen_range(-200.0..200.0), 0., 0.),
            mesh: resources.mesh.clone(),
            material: resources.material.clone(),
            ..default()
        })
        .insert(Name::new("Ship"))
        .insert(Ship);
}
