use bevy::audio::Volume;
use bevy::prelude::*;
use crate::gameplay::gameplay_loop::game_over::GameOver;

use crate::gameplay::plants::{PlantType, SpawnPlant};

pub fn spawn(
    mut commands: Commands,
    mut event: EventReader<SpawnPlant>,
    asset_server: Res<AssetServer>,
    mut event_game_over: EventWriter<GameOver>,
) {
    for event in event.read() {
        if event.0 != PlantType::Weezer {
            continue;
        }

        commands.spawn(
            AudioBundle {
                source: asset_server.load("audio/get weezered.ogg"),
                settings: PlaybackSettings::DESPAWN.with_volume(Volume::new(0.25)),
            }
        );

        event_game_over.send(GameOver("GET WEEZERED LOL"));
    }
}