use bevy::audio::Volume;
use bevy::prelude::*;
use crate::gameplay::enemies::components::Enemy;
use crate::gameplay::field::Field;
use crate::{GameState, Order};

mod ui;

#[derive(Event)]
pub struct GameOver(pub &'static str);

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<GameOver>()

            .add_systems(Update, (
                game_over_on_zombie_in_house,
            )
                .in_set(Order::GameLogic)
                .run_if(in_state(GameState::Playing)))

            .add_systems(Update, (
                on_game_over,
            ).chain()
                .in_set(Order::Cleanups)
                .run_if(in_state(GameState::Playing)))

            .add_systems(OnEnter(GameState::GameOver), ui::build_game_over_screen)
        ;
    }
}

fn game_over_on_zombie_in_house(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut event: EventWriter<GameOver>,
    zombies: Query<&Transform, With<Enemy>>,
    field: Res<Field>,
) {
    for transform in zombies.iter() {
        let position = transform.translation.x;

        if position <= field.player_house_x {
            event.send(GameOver("Pino Prime ate your brainz:("));
            
            commands.spawn(
                AudioBundle {
                    source: asset_server.load("audio/laugh.ogg"),
                    settings: PlaybackSettings::DESPAWN.with_volume(Volume::new(0.25)),
                }
            );
        }
    }
}

fn on_game_over(
    mut event: EventReader<GameOver>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for _ in event.read() {
        next_state.set(GameState::GameOver);
    }
}