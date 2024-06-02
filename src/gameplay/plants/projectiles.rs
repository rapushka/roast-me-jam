use bevy::prelude::*;
use crate::gameplay::movement::MovementSpeed;

#[derive(Component)]
pub struct Shoot(pub Time);

#[derive(Component)]
pub enum Projectile {
    Nuke,
}

pub fn move_projectiles_to_right(
    mut projectiles: Query<(&mut Transform, &MovementSpeed), With<Projectile>>,
    time: Res<Time>,
) {
    for (transform, speed) in projectiles.iter_mut() {
        transform.translation.x += speed.0 * time.delta_seconds();
    }
}


