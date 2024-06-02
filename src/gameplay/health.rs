use bevy::prelude::*;
use crate::{AppState, constants, OnAppState};
use crate::gameplay::collisions::Collision;
use crate::gameplay::enemies::components::Enemy;
use crate::gameplay::plants::time_to_live::TimeToLive;

#[derive(Event)]
pub struct Kill(pub Entity);

#[derive(Component, Reflect)]
pub struct Health(pub f32);

#[derive(Component)]
pub struct CollisionDamage(pub f32);

#[derive(Component)]
pub struct BurnDamage;

pub struct HealthPlugin;

#[derive(Component, PartialEq)]
pub enum CauseOfDeath {
    None,
    Burn,
}

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Health>()

            .add_event::<Kill>()

            .add_systems(Update, (
                apply_collision_damage,
                kill_entity_with_zero_hp,
                spawn_ash_baby,
            ).chain()
                .run_if(in_state(AppState::Gameplay)))

            .add_systems(PostUpdate, fulfill_kill_request.run_if(on_event::<Kill>()))
        ;
    }
}

fn apply_collision_damage(
    mut commands: Commands,
    mut event: EventReader<Collision>,
    damage_dealers: Query<&CollisionDamage>,
    mut enemies: Query<(Entity, &mut Health), With<Enemy>>,
    time: Res<Time>,
) {
    for e in event.read() {
        if let Ok(damage_dealer) = damage_dealers.get(e.0) {
            if let Ok((enemy, mut enemy_health)) = enemies.get_mut(e.1) {
                enemy_health.0 -= damage_dealer.0 * time.delta_seconds();

                commands.entity(enemy).insert(CauseOfDeath::Burn);
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

fn spawn_ash_baby(
    mut event: EventReader<Kill>,
    mut causes: Query<(&CauseOfDeath, &Transform)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for event in event.read() {
        if let Ok((cause_of_death, transform)) = causes.get(event.0) {
            if *cause_of_death == CauseOfDeath::Burn {
                commands.spawn(Name::new("ash baby"))
                    .insert(TimeToLive(Timer::from_seconds(constants::ASH_BABY_TTL, TimerMode::Once)))
                    .insert(SpriteBundle {
                        texture: asset_server.load("sprites/ash-baby.png"),
                        transform: Transform::from_translation(transform.translation).with_scale(Vec3::splat(0.1)),
                        ..default()
                    })
                    .insert(OnAppState(AppState::Gameplay))
                ;
            }
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