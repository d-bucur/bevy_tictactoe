use bevy::prelude::*;

use crate::AppState;
use crate::palette;

pub struct MenuPlugin;

#[derive(Component)]
struct MenuItem;

const COLOR_BG: Color = palette::SHADE_DARK;
const COLOR_BUTTON: Color = palette::SHADE_MED_LIGHT;
const COLOR_HOVER: Color = palette::SHADE_MED_DARK;
const COLOR_TEXT: Color = palette::SHADE_LIGHT;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Menu).with_system(spawn_menu))
            .add_system_set(SystemSet::on_update(AppState::Menu).with_system(menu))
            .add_system_set(SystemSet::on_exit(AppState::Menu).with_system(cleanup_menu))
            .insert_resource(ClearColor(COLOR_BG));
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
            background_color: COLOR_BUTTON.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Start",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: COLOR_TEXT,
                },
            ));
        })
        .insert(MenuItem);
}

fn menu(mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                println!("Button clicked");
            }
            Interaction::Hovered => {
                *color = COLOR_HOVER.into();
            }
            Interaction::None => {
                *color = COLOR_BUTTON.into();
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands) {}
