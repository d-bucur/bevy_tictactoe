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
    x: usize,
    y: usize,
}

#[derive(Clone, Copy)]
enum GridValue {
    X,
    O,
    Empty,
}

#[derive(Resource)]
struct Grid {
    vals: [[GridValue; 3]; 3],
}

impl Grid {
    fn new() -> Self {
        Grid {
            vals: [[GridValue::Empty; 3]; 3],
        }
    }
}

pub struct TicTacToeGamePlugin;

impl Plugin for TicTacToeGamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Grid::new())
            .add_system_set(SystemSet::on_enter(AppState::Game).with_system(setup))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(player_input));
    }
}

fn player_input(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &GridPosition, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut children_query: Query<&mut Text>,
    mut grid: ResMut<Grid>,
) {
    for (interaction, mut color, grid_position, children) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                for child in children {
                    for mut text in children_query.get_mut(*child) {
                        text.sections[0].value = "T".into();
                    }
                }
                try_place_value(&mut grid, grid_position);
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

fn try_place_value(grid: &mut ResMut<Grid>, grid_position: &GridPosition) {
    println!("Clicked {}, {}", grid_position.x, grid_position.y);
    grid.vals[grid_position.x][grid_position.y] = GridValue::O;
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let text_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 40.0,
        color: palette::SHADE_LIGHT,
    };
    let button_container = commands.spawn(make_button_container()).id();

    for i in 0..3 {
        let row_container = commands
            .spawn(make_row_container())
            .insert(PlacementButton)
            .id();
        for j in 0..3 {
            let button = commands
                .spawn(make_grid_button())
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("", text_style.clone()));
                })
                .insert(Name::new(format!("GridButton{}-{}", i, j)))
                .insert(GridPosition { x: i, y: j })
                .id();
            commands.entity(row_container).add_child(button);
            if j < 2 {
                let separator_vertical = commands.spawn(make_separator_vertical()).id();
                commands.entity(row_container).add_child(separator_vertical);
            }
        }

        commands.entity(button_container).add_child(row_container);
        if i < 2 {
            let separator_horizontal = commands.spawn(make_separator_horizontal()).id();
            commands
                .entity(button_container)
                .add_child(separator_horizontal);
        }
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
