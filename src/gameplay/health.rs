use bevy::prelude::*;
use crate::AppState;
use crate::gameplay::collisions::Collision;
use crate::gameplay::enemies::components::Enemy;

#[derive(Event)]
pub struct Kill(pub Entity);

#[derive(Component, Reflect)]
pub struct Health(pub f32);

#[derive(Component)]
pub struct CollisionDamage(pub f32);

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Health>()

            .add_event::<Kill>()

            .add_systems(Update, (
                apply_collision_damage,
                kill_entity_with_zero_hp,
            ).run_if(in_state(AppState::Gameplay)))

            .add_systems(PostUpdate, fulfill_kill_request.run_if(on_event::<Kill>()))
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
    entities: Query<(Entity, &Health), Changed<Health>>,
    mut event: EventWriter<Kill>,
) {
    for (e, health) in entities.iter() {
        if health.0 <= 0.0 {
            event.send(Kill(e));
        }
    }
}

fn fulfill_kill_request(
    mut commands: Commands,
    mut event: EventReader<Kill>,
) {
    for event in event.read() {
        commands.entity(event.0).despawn_recursive();
    }
}