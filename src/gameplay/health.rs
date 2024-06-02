use bevy::prelude::*;
use crate::AppState;
use crate::gameplay::collisions::Collision;
use crate::gameplay::enemies::components::Enemy;

#[derive(Component, Reflect)]
pub struct Health(pub f32);

#[derive(Component)]
pub struct CollisionDamage(pub f32);

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Health>()
            
            .add_systems(Update, (
                apply_collision_damage,
                kill_entity_with_zero_hp,
            ).run_if(in_state(AppState::Gameplay)))
        ;
    }
}

fn apply_collision_damage(
    mut event: EventReader<Collision>,
    damage_dealers: Query<&CollisionDamage>,
    mut enemies: Query<&mut Health, With<Enemy>>,
    time: Res<Time>,
) {
    for e in event.read() {
        if let Ok(damage_dealer) = damage_dealers.get(e.0) {
            if let Ok(mut enemy_health) = enemies.get_mut(e.1) {
                enemy_health.0 -= damage_dealer.0 * time.delta_seconds();
            }
        }
    }
}

fn kill_entity_with_zero_hp(
    mut commands: Commands,
    entities: Query<(Entity, &Health), Changed<Health>>,
) {
    for (e, health) in entities.iter() {
        if health.0 <= 0.0 {
            commands.entity(e).despawn_recursive();
        }
    }
}