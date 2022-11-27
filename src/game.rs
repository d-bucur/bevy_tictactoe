use bevy::prelude::*;

use crate::AppState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(AppState::Game).with_system(game_enter_system));
    }
}

fn game_enter_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    crate::utils::load_scene(commands, asset_server)
}