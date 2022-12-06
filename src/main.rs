mod game;
mod menu;
mod palette;
mod testmode;
mod utils;

use crate::menu::MenuPlugin;
use bevy::prelude::*;
use bevy_editor_pls::EditorPlugin;
use bevy_tweening::TweeningPlugin;
use game::TicTacToeGamePlugin;
use testmode::TestModePlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Menu,
    Game,
    Test,
    GameOver,
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
        .add_plugin(TicTacToeGamePlugin)
        .add_plugin(TestModePlugin)
        .add_plugin(TweeningPlugin)
        // .add_plugin(EditorPlugin)
        .add_state(AppState::Menu)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
