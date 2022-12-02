use bevy::prelude::*;

use crate::{palette, AppState};

#[derive(Component)]
struct PlacementButton;

#[derive(Component)]
struct GridPosition {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy)]
enum PlayerTurn {
    X,
    O,
}

impl PlayerTurn {
    fn next(&self) -> PlayerTurn {
        match *self {
            PlayerTurn::O => PlayerTurn::X,
            PlayerTurn::X => PlayerTurn::O,
        }
    }
}

impl From<PlayerTurn> for String {
    fn from(val: PlayerTurn) -> Self {
        match val {
            PlayerTurn::X => "X".into(),
            PlayerTurn::O => "O".into(),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum GridValue {
    X,
    O,
    Empty,
}

impl From<PlayerTurn> for GridValue {
    fn from(val: PlayerTurn) -> Self {
        match val {
            PlayerTurn::O => GridValue::O,
            PlayerTurn::X => GridValue::X,
        }
    }
}

#[derive(Resource)]
struct GameState {
    player_turn: PlayerTurn,
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

    fn get(&self, pos: &GridPosition) -> GridValue {
        return self.vals[pos.x][pos.y];
    }

    fn set(&mut self, pos: &GridPosition, val: GridValue) {
        self.vals[pos.x][pos.y] = val
    }
}

#[derive(Component)]
struct StatusText;

struct StatusTextUpdate {
    text: String,
}

pub struct TicTacToeGamePlugin;

impl Plugin for TicTacToeGamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Grid::new())
            .insert_resource(GameState {
                player_turn: PlayerTurn::X,
            })
            .add_system_set(SystemSet::on_enter(AppState::Game).with_system(setup))
            .add_system_set(
                SystemSet::on_update(AppState::Game)
                    .with_system(player_input)
                    .with_system(update_status),
            )
            .add_event::<StatusTextUpdate>();
    }
}

fn player_input(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &GridPosition, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut children_query: Query<&mut Text>,
    mut grid: ResMut<Grid>,
    mut game_state: ResMut<GameState>,
    mut status_writer: EventWriter<StatusTextUpdate>,
) {
    for (interaction, mut color, grid_position, children) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                // TODO only check input and buttons in this system, set variable and do gameplay logic in another system
                if grid.get(grid_position) == GridValue::Empty {
                    // TODO write helper function to get component from children
                    for child in children {
                        for mut text in children_query.get_mut(*child) {
                            text.sections[0].value = game_state.player_turn.into()
                        }
                    }
                    grid.set(grid_position, game_state.player_turn.into());
                    game_state.player_turn = game_state.player_turn.next();
                    status_writer.send(StatusTextUpdate {
                        text: format!("{} to move", String::from(game_state.player_turn)),
                    })
                } else {
                    status_writer.send(StatusTextUpdate {
                        text: "Invalid move".into(),
                    })
                }
            }
            Interaction::Hovered => {
                *color = palette::SHADE_MED_LIGHT.into();
            }
            Interaction::None => {
                *color = palette::SHADE_MED_DARK.into();
            }
        }
    }
}

fn update_status(
    mut status_events: EventReader<StatusTextUpdate>,
    mut query: Query<&mut Text, With<StatusText>>,
) {
    for event in status_events.iter() {
        let mut res = query.get_single_mut().unwrap();
        res.sections[0].value = event.text.clone();
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let text_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 40.0,
        color: palette::SHADE_LIGHT,
    };
    let button_container = commands.spawn(make_button_container()).id();

    // create rows for buttons
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

        // create cells in rows
        commands.entity(button_container).add_child(row_container);
        if i < 2 {
            let separator_horizontal = commands.spawn(make_separator_horizontal()).id();
            commands
                .entity(button_container)
                .add_child(separator_horizontal);
        }
    }

    // place buttons and helper text
    let canvas = commands.spawn(make_canvas_node()).id();
    let helper_text = commands
        .spawn(TextBundle::from_section("", text_style.clone()))
        .insert(StatusText)
        .id();
    commands.entity(canvas).add_child(button_container);
    commands.entity(canvas).add_child(helper_text);
}

fn make_canvas_node() -> NodeBundle {
    NodeBundle {
        style: Style {
            margin: UiRect::all(Val::Auto),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            size: Size::new(Val::Px(200.0), Val::Percent(100.0)),
            ..default()
        },
        ..default()
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
