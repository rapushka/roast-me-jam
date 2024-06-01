use crate::gameplay::seeds::initialization::spawn_seeds_slots;
use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::{AppState, constants};
use crate::gameplay::enemies::components::Enemy;
use crate::ui::Clicked;

mod initialization;
mod components;

pub struct SeedsPlugin;

impl Plugin for SeedsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Gameplay), spawn_seeds_slots)
        ;
    }
}

