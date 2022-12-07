use bevy::prelude::*;

use crate::{
    game::{AIPauseTimer, Grid, GridPosition, GridValue, StatusTextUpdateEvent, TryPlaceEvent},
    players::*,
};

pub struct AIPlugin;

impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(PlayerDriver::AI).with_system(start_ai_move))
            .add_system_set(SystemSet::on_update(PlayerDriver::AI).with_system(handle_ai_move));
    }
}

fn start_ai_move(mut status_writer: EventWriter<StatusTextUpdateEvent>, mut commands: Commands) {
    // TODO place piece randomly
    status_writer.send(StatusTextUpdateEvent {
        text: "AI thinking".into(),
    });
    commands.spawn(AIPauseTimer(Timer::from_seconds(1., TimerMode::Once)));
}

// TODO use RunCriteria instead?
fn handle_ai_move(
    grid: Res<Grid>,
    mut place_writer: EventWriter<TryPlaceEvent>,
    mut timer_query: Query<&mut AIPauseTimer>,
    time: Res<Time>,
) {
    for mut timer in &mut timer_query {
        if timer.0.tick(time.delta()).just_finished() {
            for (x, _) in grid.vals.iter().enumerate() {
                for (y, &val) in grid.vals[x].iter().enumerate() {
                    if val == GridValue::Empty {
                        place_writer.send(TryPlaceEvent {
                            pos: GridPosition { x: x, y: y },
                        });
                        return;
                    }
                }
            }
        }
    }
}
