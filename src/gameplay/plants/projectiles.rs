use bevy::prelude::*;
use crate::gameplay::movement::MovementSpeed;
use crate::{AppState, constants, OnAppState};
use crate::gameplay::collisions::components::CircleCollider;
use crate::gameplay::health::{BurnDamage, CollisionDamage};
use crate::gameplay::plants::time_to_live::TimeToLive;

#[derive(Component)]
pub struct Shooter {
    pub timer: Timer,
    pub projectile: Projectile,
    pub offset: Vec2,
}

#[derive(Event)]
pub struct Shoot {
    pub projectile: Projectile,
    pub position: Vec3,
}

#[derive(Component, Clone, Copy)]
pub enum Projectile {
    Nuke,
}

pub fn move_projectiles_to_right(
    mut projectiles: Query<(&mut Transform, &MovementSpeed), With<Projectile>>,
    time: Res<Time>,
) {
    for (mut transform, speed) in projectiles.iter_mut() {
        transform.translation.x += speed.0 * time.delta_seconds();
    }
}

pub fn shoot_periodically(
    mut shooters: Query<(&mut Shooter, &GlobalTransform)>,
    time: Res<Time>,
    mut shoot_event: EventWriter<Shoot>,
) {
    for (mut shooter, transform) in shooters.iter_mut() {
        shooter.timer.tick(time.delta());
        let origin = transform.translation();

        if shooter.timer.finished() {
            let offset = shooter.offset;
            let offset = Vec3::new(offset.x, offset.y, 0.0);

            shoot_event.send(Shoot {
                position: origin + offset,
                projectile: shooter.projectile,
            });
        }
    }
}

pub fn shoot(
    mut commands: Commands,
    mut shoot_event: EventReader<Shoot>,
    asset_server: Res<AssetServer>,
) {
    for shoot in shoot_event.read() {
        commands.spawn(Name::new("projectile"))
            .insert(shoot.projectile)
            .insert(SpriteBundle {
                texture: asset_server.load(get_sprite(shoot.projectile)),
                transform: Transform::from_translation(shoot.position).with_scale(get_scale(shoot.projectile)),
                ..default()
            })
            .insert(OnAppState(AppState::Gameplay))
            .insert(TimeToLive(Timer::from_seconds(20.0, TimerMode::Once)))
            .insert(MovementSpeed(get_movement_speed(shoot.projectile)))
            .insert(CircleCollider::new(25.0))
            .insert(CollisionDamage(get_damage(shoot.projectile)))
            .insert(BurnDamage)
        ;
    }
}

pub fn get_sprite(projectile: Projectile) -> &'static str {
    match projectile {
        Projectile::Nuke => "sprites/plants/nuke.png",
    }
}

pub fn get_scale(projectile: Projectile) -> Vec3 {
    match projectile {
        Projectile::Nuke => Vec3::splat(0.1),
    }
}

pub fn get_movement_speed(projectile: Projectile) -> f32 {
    match projectile {
        Projectile::Nuke => constants::NUKE_MOVEMENT_SPEED,
    }
}

pub fn get_damage(projectile: Projectile) -> f32 {
    match projectile {
        Projectile::Nuke => constants::NUKE_DAMAGE,
    }
}