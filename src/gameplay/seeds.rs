use bevy::app::{App, Plugin};
use bevy::prelude::*;

use crate::AppState;
use crate::gameplay::plants::plant_preview::PlantPreviewPlugin;
use crate::gameplay::seeds::initialization::*;
use crate::gameplay::seeds::planting::PlantingPlugin;

pub mod initialization;
pub mod components;
pub mod planting;

pub struct SeedsPlugin;

impl Plugin for SeedsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                PlantingPlugin,
                PlantPreviewPlugin,
            ))

            .add_systems(OnEnter(AppState::Gameplay), (
                spawn_seeds_slots,
            ))

            .add_systems(Update, (
                fill_seed_slots,
                seed_preview
            ).chain())
        ;
    }
}