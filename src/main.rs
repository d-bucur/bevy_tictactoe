mod menu;
mod game;
mod palette;
mod utils;

use bevy::prelude::*;
use bevy_editor_pls::EditorPlugin;
use game::GamePlugin;
use crate::menu::MenuPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Menu,
    Game,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Bevy test".to_string(),
                width: 1024.,
                height: 600.,
                present_mode: bevy::window::PresentMode::AutoVsync,
                ..default()
            },
            ..default()
        }))
        .add_plugin(MenuPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(EditorPlugin)
        .add_state(AppState::Menu)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
