use std::{collections::HashSet, time::Duration};

use bevy::prelude::*;
use bevy_tweening::{Animator, EaseFunction, Lens, Tracks, Tween};

use crate::{palette, AppState};

const COLOR_SEPARATOR: Color = palette::SHADE_LIGHT;
const COLOR_TEXT: Color = palette::SHADE_LIGHT;
const COLOR_BUTTON: Color = palette::SHADE_DARK;
const COLOR_HIGHLIGHT: Color = palette::SHADE_MED_LIGHT;
const TEXT_SIZE: f32 = 40.;

// Components
#[derive(Component)]
struct PlacementButton;

#[derive(Component, Clone, Copy)]
struct GridPosition {
    x: usize,
    y: usize,
}

#[derive(Component)]
struct StatusText;

// Game structures
#[derive(Clone, Copy, Default)]
enum PlayerTurn {
    #[default]
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

impl GridValue {
    fn score(&self) -> i32 {
        match self {
            GridValue::X => 1,
            GridValue::O => -1,
            GridValue::Empty => 0,
        }
    }
}

impl From<PlayerTurn> for GridValue {
    fn from(val: PlayerTurn) -> Self {
        match val {
            PlayerTurn::O => GridValue::O,
            PlayerTurn::X => GridValue::X,
        }
    }
}

// Resources
#[derive(Resource, Default)]
struct GameState {
    player_turn: PlayerTurn,
}

#[derive(Component)]
struct GameOverTimer(Timer);

#[derive(Component)]
struct GameScene;

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

    fn get(&self, pos: GridPosition) -> GridValue {
        return self.vals[pos.x][pos.y];
    }

    fn set(&mut self, pos: GridPosition, val: GridValue) {
        self.vals[pos.x][pos.y] = val
    }
}

// Events
struct StatusTextUpdateEvent {
    text: String,
}

struct TryPlaceEvent {
    pos: GridPosition,
    text_entity: Entity,
}

struct PiecePlacedEvent {
    pos: GridPosition,
}

struct WinState {
    player: PlayerTurn,
    victory_cells: HashSet<(usize, usize)>,
}

enum GameEndedEvent {
    Draw,
    Win(WinState),
}

// Plugin
pub struct TicTacToeGamePlugin;

impl Plugin for TicTacToeGamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Grid::new())
            .insert_resource(GameState::default())
            .add_system_set(SystemSet::on_enter(AppState::Game).with_system(setup))
            .add_system_set(
                SystemSet::on_update(AppState::Game)
                    .with_system(handle_player_input)
                    .with_system(update_status_text)
                    .with_system(place_grid_piece)
                    .with_system(check_win_condition)
                    .with_system(handle_game_over),
            )
            .add_system_set(
                SystemSet::on_update(AppState::GameOver).with_system(update_status_text),
            )
            .add_system_set(SystemSet::on_exit(AppState::GameOver).with_system(cleanup))
            .add_system(check_timer)
            .add_event::<StatusTextUpdateEvent>()
            .add_event::<TryPlaceEvent>()
            .add_event::<PiecePlacedEvent>()
            .add_event::<GameEndedEvent>();
    }
}

// Systems
fn handle_player_input(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &GridPosition, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut place_writer: EventWriter<TryPlaceEvent>,
) {
    for (interaction, mut color, &grid_position, children) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                place_writer.send(TryPlaceEvent {
                    pos: grid_position,
                    text_entity: *children.iter().next().unwrap(),
                });
            }
            Interaction::Hovered => {
                *color = COLOR_HIGHLIGHT.into();
            }
            Interaction::None => {
                *color = COLOR_BUTTON.into();
            }
        }
    }
}

fn update_status_text(
    mut status_events: EventReader<StatusTextUpdateEvent>,
    mut query: Query<&mut Text, With<StatusText>>,
) {
    for event in status_events.iter() {
        let mut res = query.get_single_mut().unwrap();
        res.sections[0].value = event.text.clone();
    }
}

fn place_grid_piece(
    mut try_place_events: EventReader<TryPlaceEvent>,
    mut grid: ResMut<Grid>,
    mut game_state: ResMut<GameState>,
    mut text_query: Query<&mut Text>,
    mut status_writer: EventWriter<StatusTextUpdateEvent>,
    mut placed_piece_writer: EventWriter<PiecePlacedEvent>,
    mut commands: Commands,
) {
    for event in try_place_events.iter() {
        if grid.get(event.pos) == GridValue::Empty {
            let mut text = text_query
                .get_component_mut::<Text>(event.text_entity)
                .unwrap();
            text.sections[0].value = game_state.player_turn.into();
            grid.set(event.pos, game_state.player_turn.into());
            game_state.player_turn = game_state.player_turn.next();
            placed_piece_writer.send(PiecePlacedEvent { pos: event.pos });
            
            status_writer.send(StatusTextUpdateEvent {
                text: format!("{} to move", String::from(game_state.player_turn)),
            });
            // tween piece into position
            let animator = Animator::new(Tracks::new([
                Tween::new(
                    EaseFunction::QuadraticOut,
                    Duration::from_millis(250),
                    bevy_tweening::lens::TransformScaleLens {
                        start: Vec3::ONE * 3.,
                        end: Vec3::ONE,
                    },
                ),
                Tween::new(
                    EaseFunction::QuadraticOut,
                    Duration::from_millis(250),
                    bevy_tweening::lens::TransformRotateZLens { start: 1., end: 0. },
                ),
            ]));
            commands.entity(event.text_entity).insert(animator);
        } else {
            status_writer.send(StatusTextUpdateEvent {
                text: "Invalid move".into(),
            })
        }
    }
}

fn check_win_condition(
    mut placed_piece_reader: EventReader<PiecePlacedEvent>,
    grid: Res<Grid>,
    mut game_ended_writer: EventWriter<GameEndedEvent>,
) {
    for event in placed_piece_reader.iter() {
        let mut winning_pos: HashSet<(usize, usize)> = HashSet::new();
        let horizontal: i32 = (0..3).map(|x| grid.vals[x][event.pos.y].score()).sum();
        let vertical: i32 = (0..3).map(|y| grid.vals[event.pos.x][y].score()).sum();
        let diagonal_one: i32 = (0..3).map(|i| grid.vals[i][i].score()).sum();
        let diagonal_two: i32 = (0..3).map(|i| grid.vals[2 - i][i].score()).sum();

        if horizontal.abs() == 3 {
            winning_pos.extend((0..3).zip(std::iter::repeat(event.pos.y)))
        }
        if vertical.abs() == 3 {
            winning_pos.extend(std::iter::repeat(event.pos.x).zip(0..3))
        }
        if diagonal_one.abs() == 3 {
            winning_pos.extend((0..3).zip(0..3))
        }
        if diagonal_two.abs() == 3 {
            winning_pos.extend((0..3).zip((0..3).rev()))
        }

        if horizontal == 3 || vertical == 3 || diagonal_one == 3 || diagonal_two == 3 {
            game_ended_writer.send(GameEndedEvent::Win(WinState {
                player: PlayerTurn::X,
                victory_cells: winning_pos,
            }));
            return
        } else if horizontal == -3 || vertical == -3 || diagonal_one == -3 || diagonal_two == -3 {
            game_ended_writer.send(GameEndedEvent::Win(WinState {
                player: PlayerTurn::O,
                victory_cells: winning_pos,
            }));
            return
        }
        
        let is_draw = grid
            .vals
            .iter()
            .all(|row| row.iter().all(|&cell| cell != GridValue::Empty));
        if is_draw {
            game_ended_writer.send(GameEndedEvent::Draw);
            return
        }
    }
}

fn handle_game_over(
    mut game_ended_reader: EventReader<GameEndedEvent>,
    mut status_writer: EventWriter<StatusTextUpdateEvent>,
    mut state: ResMut<State<AppState>>,
    mut commands: Commands,
    mut query: Query<(&mut BackgroundColor, &GridPosition)>,
) {
    for event in game_ended_reader.iter() {
        let message = match event {
            GameEndedEvent::Draw => "It's a draw!",
            GameEndedEvent::Win(win_state) => {
                for (mut color, position) in &mut query {
                    if win_state.victory_cells.contains(&(position.x, position.y)) {
                        *color = COLOR_HIGHLIGHT.into();
                    }
                }
                match win_state.player {
                    PlayerTurn::X => "X wins!",
                    PlayerTurn::O => "O wins!",
                }
            }
        };
        status_writer.send(StatusTextUpdateEvent {
            text: message.into(),
        });
        state.set(AppState::GameOver).unwrap();
        commands.spawn(GameOverTimer(Timer::from_seconds(5., TimerMode::Once)));
    }
}

fn check_timer(
    time: Res<Time>,
    mut query: Query<&mut GameOverTimer>,
    mut state: ResMut<State<AppState>>,
) {
    for mut timer in &mut query {
        if timer.0.tick(time.delta()).just_finished() {
            println!("finished timer. Restarting");
            state.set(AppState::Menu).unwrap();
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let text_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: TEXT_SIZE,
        color: COLOR_TEXT,
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
    let canvas = commands.spawn(make_canvas_node()).insert(GameScene).id();
    let mut helper_text_bundle = TextBundle::from_section("", text_style.clone());
    helper_text_bundle.style.size = Size::new(Val::Auto, Val::Px(50.));
    let helper_text = commands.spawn(helper_text_bundle).insert(StatusText).id();
    commands.entity(canvas).add_child(button_container);
    commands.entity(canvas).add_child(helper_text);
}

fn cleanup(mut commands: Commands, query: Query<Entity, With<GameScene>>) {
    println!("cleaning up");
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    commands.insert_resource(Grid::new());
    commands.insert_resource(GameState::default());
}

// Helpers for bundles
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
        background_color: COLOR_BUTTON.into(),
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
        background_color: COLOR_BUTTON.into(),
        ..default()
    }
}

fn make_row_container() -> NodeBundle {
    NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Row,
            // justify_content: JustifyContent::Center,
            // align_items: AlignItems::Center,
            size: Size::new(Val::Px(200.0), Val::Px(200.0)),
            ..default()
        },
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
        background_color: COLOR_SEPARATOR.into(),
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
        background_color: COLOR_SEPARATOR.into(),
        ..default()
    }
}
