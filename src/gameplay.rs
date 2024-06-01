use bevy::prelude::*;
use crate::gameplay::animations::animate_sprites;

use crate::gameplay::background::BackgroundPlugin;
use crate::gameplay::enemies::EnemiesPlugin;
use crate::gameplay::gameplay_loop::GameplayLoopPlugin;

mod gameplay_loop;
mod background;
mod enemies;
mod animations;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                BackgroundPlugin,
                GameplayLoopPlugin,
                EnemiesPlugin,
            ))

            .add_systems(Update, animate_sprites)
        ;
    }
}


