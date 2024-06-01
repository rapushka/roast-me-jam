use bevy::prelude::*;
use crate::AppState;
use crate::gameplay::animations::animate_sprites;

use crate::gameplay::background::BackgroundPlugin;
use crate::gameplay::enemies::EnemiesPlugin;
use crate::gameplay::field::init_field;
use crate::gameplay::gameplay_loop::GameplayLoopPlugin;

mod gameplay_loop;
mod background;
mod enemies;
mod animations;
mod field;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                BackgroundPlugin,
                GameplayLoopPlugin,
                EnemiesPlugin,
            ))

            .add_systems(OnEnter(AppState::Loading), init_field)

            .add_systems(Update, animate_sprites)
        ;
    }
}


