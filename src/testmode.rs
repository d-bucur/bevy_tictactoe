use bevy::prelude::*;

use crate::AppState;

pub struct TestModePlugin;

impl Plugin for TestModePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(SystemSet::on_enter(AppState::Test).with_system(setup));
    }
}

fn setup() {
    println!("bulding test");
}
