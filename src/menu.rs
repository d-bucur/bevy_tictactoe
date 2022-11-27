use bevy::prelude::*;

use crate::AppState;

pub struct MenuPlugin;

#[derive(Component)]
struct MenuItem;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Menu).with_system(spawn_menu))
            .add_system_set(SystemSet::on_update(AppState::Menu).with_system(menu))
            .add_system_set(SystemSet::on_exit(AppState::Menu).with_system(cleanup_menu));
    }
}

fn spawn_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Percent(10.), Val::Percent(10.)),
                // center button
                margin: UiRect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Start",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::RED,
                },
            ));
        })
        .insert(MenuItem);
}

fn menu(mut interaction_query: Query<(&Interaction), (Changed<Interaction>, With<Button>)>) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                println!("Button clicked");
            }
            _ => {}
        }
    }
}

fn cleanup_menu(mut commands: Commands) {}