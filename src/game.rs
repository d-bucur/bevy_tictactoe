use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{
    palette::{self, SHADE_DARK},
    AppState,
};
use rand::Rng;

#[derive(Component)]
struct PlacementButton;

#[derive(Component)]
struct GridPosition {
    x: u8,
    y: u8,
}

#[derive(Resource, Default)]
struct GameResources {
    text_style: TextStyle,
    button: ButtonBundle,
}

pub struct TicTacToeGamePlugin;

impl Plugin for TicTacToeGamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameResources::default())
            .add_system_set(SystemSet::on_enter(AppState::Game).with_system(setup))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(player_input));
    }
}

fn player_input(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                println!("Clicked")
            }
            Interaction::Hovered => {
                *color = palette::SHADE_MED_LIGHT.into();
            }
            Interaction::None => {
                *color = palette::SHADE_MED_DARK.into();
            }
            _ => (),
        }
    }
}

fn setup(
    mut commands: Commands,
    mut resources: ResMut<GameResources>,
    asset_server: Res<AssetServer>,
) {
    // TODO no need for resources. remove
    resources.text_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 40.0,
        color: palette::SHADE_LIGHT,
    };
    resources.button = make_grid_button();
    let button_container = commands.spawn(make_button_container()).id();

    for i in 0..3 {
        let row_container = commands
            .spawn(make_row_container())
            .insert(PlacementButton)
            .id();
        for j in 0..3 {
            let button = commands
                .spawn(resources.button.clone())
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("", resources.text_style.clone()));
                })
                .insert(Name::new("Button"))
                .insert(GridPosition { x: i, y: j })
                .id();
            let separator_vertical = commands.spawn(make_separator_vertical()).id();
            commands.entity(row_container).add_child(button);
            commands.entity(row_container).add_child(separator_vertical);
        }

        let separator_horizontal = commands.spawn(make_separator_horizontal()).id();
        commands.entity(button_container).add_child(row_container);
        commands
            .entity(button_container)
            .add_child(separator_horizontal);
    }
}

fn make_grid_button() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Percent(100.), Val::Percent(100.)),
            // center button
            margin: UiRect::all(Val::Auto),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: palette::SHADE_DARK.into(),
        ..default()
    }
}

fn make_button_container() -> NodeBundle {
    NodeBundle {
        style: Style {
            margin: UiRect::all(Val::Auto),
            flex_direction: FlexDirection::Column,
            size: Size::new(Val::Px(200.0), Val::Px(200.0)),
            ..default()
        },
        background_color: palette::SHADE_DARK.into(),
        ..default()
    }
}

fn make_row_container() -> NodeBundle {
    NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            size: Size::new(Val::Px(200.0), Val::Px(200.0)),
            ..default()
        },
        background_color: palette::SHADE_MED_DARK.into(),
        ..default()
    }
}

fn make_separator_vertical() -> NodeBundle {
    NodeBundle {
        style: Style {
            margin: UiRect::all(Val::Auto),
            flex_direction: FlexDirection::Column,
            size: Size::new(Val::Px(10.0), Val::Percent(100.0)),
            ..default()
        },
        background_color: palette::SHADE_LIGHT.into(),
        ..default()
    }
}

fn make_separator_horizontal() -> NodeBundle {
    NodeBundle {
        style: Style {
            margin: UiRect::all(Val::Auto),
            flex_direction: FlexDirection::Column,
            size: Size::new(Val::Percent(100.0), Val::Px(10.0)),
            ..default()
        },
        background_color: palette::SHADE_LIGHT.into(),
        ..default()
    }
}
