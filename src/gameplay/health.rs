use bevy::prelude::*;
use crate::AppState;
use crate::gameplay::collisions::Collision;
use crate::gameplay::enemies::components::Enemy;

#[derive(Component)]
pub struct Health(pub f32);

#[derive(Component)]
pub struct CollisionDamage(pub f32);

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                apply_collision_damage
            ).run_if(in_state(AppState::Gameplay)))
        ;
    }
}

pub fn apply_collision_damage(
    mut event: EventReader<Collision>,
    damage_dealers: Query<&CollisionDamage>,
    mut enemies: Query<&mut Health, With<Enemy>>,
) {
    for e in event.read() {
        if let Ok(damage_dealer) = damage_dealers.get(e.0) {
            if let Ok(mut enemy_health) = enemies.get_mut(e.1) {
                enemy_health.0 -= damage_dealer.0;
            }
        }
    }
}