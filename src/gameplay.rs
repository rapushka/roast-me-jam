use bevy::prelude::*;
use crate::gameplay::background::BackgroundPlugin;

mod background;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(BackgroundPlugin)
        ;
    }
}
