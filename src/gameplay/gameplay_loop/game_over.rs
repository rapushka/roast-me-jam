use bevy::prelude::*;
use crate::gameplay::enemies::components::Enemy;
use crate::gameplay::field::Field;
use crate::GameState;

#[derive(Event)]
pub struct GameOver(pub &'static str);

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<GameOver>()

            .add_systems(Update, (
                game_over_on_zombie_in_house,
            ).run_if(in_state(GameState::Playing)))

            .add_systems(Update, (
                on_game_over,
            ).run_if(in_state(GameState::Playing)))
        ;
    }
}

fn game_over_on_zombie_in_house(
    mut event: EventWriter<GameOver>,
    zombies: Query<&Transform, With<Enemy>>,
    field: Res<Field>,
) {
    for transform in zombies.iter() {
        let position = transform.translation.x;

        if position <= field.player_house_x {
            event.send(GameOver("Pino Prime ate your brainz:("));
        }
    }
}

fn on_game_over(
    mut event: EventReader<GameOver>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for e in event.read() {
        println!("died cuz: {}", e.0);
        next_state.set(GameState::GameOver);
    }
}