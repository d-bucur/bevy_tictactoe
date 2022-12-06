use bevy::{prelude::*, utils::HashMap};

use crate::{game::PlayerTurn};

#[derive(Clone, Eq, PartialEq, Debug, Hash, Copy)]
pub(crate) enum PlayerDriver {
    Input,
    AI,
    None,
}

#[derive(Resource)]
pub struct PlayerDrivers(pub(crate) HashMap<PlayerTurn, PlayerDriver>);
