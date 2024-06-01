use bevy::prelude::*;

use crate::gameplay::background::BackgroundPlugin;
use crate::gameplay::gameplay_loop::GameplayLoopPlugin;

mod gameplay_loop;
mod background;
mod enemies;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                BackgroundPlugin,
                GameplayLoopPlugin
            ))
        ;
    }
}


