use bevy::{prelude::*};
use bevy_inspector_egui::{WorldInspectorPlugin, Inspectable, RegisterInspectable};
use bevy_editor_pls::*;

#[derive(Component)]
struct Person;

#[derive(Component, Inspectable)]
struct Name(String);

#[derive(Resource)]
struct GreetTimer(Timer);

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(add_people)
            .add_system(greet_people)
            .insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
    }
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Mary".to_string())));
    commands.spawn((Person, Name("Renzo".to_string())));
    commands.spawn((Person, Name("Elaina".to_string())));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("hello {}!", name.0);
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        // .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(EditorPlugin)
        .register_inspectable::<Name>()
        .run();
}