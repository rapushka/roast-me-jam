use bevy::prelude::*;

use crate::gameplay::enemies::components::Enemy;
use crate::GameState;

#[derive(Component)]
pub struct MovementSpeed(pub f32);

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, move_zombies_to_left.run_if(in_state(GameState::Playing)))
        ;
    }
}

fn move_zombies_to_left(
    mut zombies: Query<(&mut Transform, &MovementSpeed), With<Enemy>>,
    time: Res<Time>,
) {
    for (mut transform, speed) in zombies.iter_mut() {
        transform.translation.x -= speed.0 * time.delta_seconds();
    }
}