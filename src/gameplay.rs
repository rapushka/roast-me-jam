use bevy::prelude::*;
use crate::AppState;
use crate::gameplay::animations::animate_sprites;

use crate::gameplay::background::BackgroundPlugin;
use crate::gameplay::collisions::CollisionsPlugin;
use crate::gameplay::enemies::EnemiesPlugin;
use crate::gameplay::field::init_field;
use crate::gameplay::gameplay_loop::GameplayLoopPlugin;
use crate::gameplay::movement::MovementPlugin;
use crate::gameplay::plants::PlantsPlugin;
use crate::gameplay::seeds::*;

pub(crate) mod gameplay_loop;
mod background;
mod enemies;
mod animations;
mod field;
mod movement;
mod seeds;
mod plants;
mod collisions;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                BackgroundPlugin,
                GameplayLoopPlugin,
                EnemiesPlugin,
                MovementPlugin,
                SeedsPlugin,
                PlantsPlugin,
                CollisionsPlugin,
            ))

            .add_systems(OnEnter(AppState::Loading), init_field)

            .add_systems(Update, animate_sprites)
        ;
    }
}


