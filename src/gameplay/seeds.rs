use bevy::app::{App, Plugin};
use bevy::prelude::*;

use crate::AppState;
use crate::gameplay::seeds::initialization::*;

mod initialization;
mod components;

pub struct SeedsPlugin;

impl Plugin for SeedsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Gameplay), (
                spawn_seeds_slots,
            ))
        ;
    }
}