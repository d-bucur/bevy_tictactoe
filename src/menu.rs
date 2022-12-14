use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::game::PlayerTurn;
use crate::palette;
use crate::players::*;
use crate::AppState;
use crate::PlayerDriver;

pub struct MenuPlugin;

#[derive(Component)]
struct MenuItem;

#[derive(Component)]
struct PlayButton;

#[derive(Component)]
struct TestButton;

const COLOR_BG: Color = palette::SHADE_DARK;
const COLOR_BUTTON: Color = palette::SHADE_MED_LIGHT;
const COLOR_HOVER: Color = palette::SHADE_MED_DARK;
const COLOR_TEXT: Color = palette::SHADE_LIGHT;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Menu)
                .with_system(spawn_menu)
                .label("spawn"),
        )
        .add_system_set(SystemSet::on_update(AppState::Menu).with_system(menu))
        // .add_system_set(
        //     SystemSet::on_exit(AppState::Menu)
        //         .with_system(save_scene)
        //         .before("clean"),
        // )
        .add_system_set(
            SystemSet::on_exit(AppState::Menu)
                .with_system(cleanup_menu)
                .label("clean"),
        )
        .insert_resource(ClearColor(COLOR_BG));
    }
}

fn spawn_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let button_style = Style {
        // size: Size::new(Val::Percent(75.), Val::Px(50.)),
        // center button
        margin: UiRect::bottom(Val::Px(10.)),
        // horizontally center child text
        justify_content: JustifyContent::Center,
        // vertically center child text
        align_items: AlignItems::Center,
        ..default()
    };
    let text_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 40.0,
        color: COLOR_TEXT,
    };
    let button_container = commands
        .spawn(NodeBundle {
            style: Style {
                margin: UiRect::all(Val::Auto),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .insert(MenuItem)
        .id();

    let start_button = commands
        .spawn(ButtonBundle {
            style: button_style.clone(),
            background_color: COLOR_BUTTON.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section("2 Players", text_style.clone()));
        })
        .insert(PlayButton)
        .id();

    let test_button = commands
        .spawn(ButtonBundle {
            style: button_style,
            background_color: COLOR_BUTTON.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section("VS AI", text_style));
        })
        .insert(TestButton) // TODO rename
        .id();

    commands.entity(button_container).add_child(start_button);
    commands.entity(button_container).add_child(test_button);
}

fn menu(
    mut app_state: ResMut<State<AppState>>,
    mut current_player_driver: ResMut<State<PlayerDriver>>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            Option<&PlayButton>,
            Option<&TestButton>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut commands: Commands,
) {
    for (interaction, mut color, play, test) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                if play.is_some() {
                    app_state.set(AppState::Game).unwrap();
                    commands.insert_resource(PlayerDrivers(HashMap::from([
                        (PlayerTurn::X, PlayerDriver::Input),
                        (PlayerTurn::O, PlayerDriver::Input),
                    ])));
                    current_player_driver.overwrite_set(PlayerDriver::Input);
                }
                if test.is_some() {
                    app_state.set(AppState::Game).unwrap();
                    commands.insert_resource(PlayerDrivers(HashMap::from([
                        (PlayerTurn::X, PlayerDriver::Input), // TODO two AIs don't work
                        (PlayerTurn::O, PlayerDriver::AI),
                    ])));
                    current_player_driver.overwrite_set(PlayerDriver::Input); // TODO use first player type. maybe do in game setup
                }
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

// TODO use generic system for cleanup
fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MenuItem>>) {
    for item in &query {
        commands.entity(item).despawn_recursive();
    }
}

fn save_scene(world: &World) {
    crate::utils::save_to_scene(world);
}
