use std::time::Duration;
use bevy::audio::Volume;

use bevy::prelude::*;
use rand::Rng;

use crate::{AppState, constants, OnAppState};
use crate::controls::Clickable;
use crate::gameplay::collisions::components::CircleCollider;
use crate::gameplay::field::Field;
use crate::gameplay::movement::move_to_target::MoveToTarget;
use crate::gameplay::movement::MovementSpeed;
use crate::gameplay::plants::price::current_money::CurrentMoney;
use crate::gameplay::plants::price::money_rain::seed_rain::{DropSeed, SeedRainPlugin};
use crate::gameplay::plants::time_to_live::TimeToLive;
use crate::ui::Clicked;

pub mod seed_rain;

#[derive(Event)]
pub struct DropMoney;

#[derive(Event)]
pub struct SpawnMoney {
    pub start_position: Vec3,
    pub end_position: Vec3,
}

#[derive(Resource)]
pub struct MoneyRain(pub Timer);

#[derive(Component)]
pub struct MoneyDroplet;

pub struct MoneyRainPlugin;

impl Plugin for MoneyRainPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(SeedRainPlugin)

            .add_event::<DropMoney>()
            .add_event::<SpawnMoney>()

            .add_systems(OnEnter(AppState::Gameplay), start_money_rain)
            .add_systems(Update, (
                tick_money_rain,
                drop_money_rain,
                pick_money_droplet,
                spawn_money,
            ).chain()
                .run_if(in_state(AppState::Gameplay)))
            .add_systems(OnExit(AppState::Gameplay), stop_money_rain)
        ;
    }
}

fn start_money_rain(
    mut commands: Commands,
) {
    let mut rng = rand::thread_rng();
    let duration = rng.gen_range(constants::MONEY_RAIN_FREQUENCY_RANGE);

    let money_rain_timer = MoneyRain(Timer::from_seconds(duration, TimerMode::Once));

    commands.insert_resource(money_rain_timer);
}

fn stop_money_rain(
    mut commands: Commands,
) {
    commands.remove_resource::<MoneyRain>();
}

fn tick_money_rain(
    mut money_rain: ResMut<MoneyRain>,
    time: Res<Time>,
    mut event: EventWriter<DropMoney>,
) {
    let mut timer = &mut money_rain.0;
    timer.tick(time.delta());

    if timer.finished() {
        let mut rng = rand::thread_rng();
        let duration = rng.gen_range(constants::MONEY_RAIN_FREQUENCY_RANGE);

        timer.reset();
        timer.set_duration(Duration::from_secs_f32(duration));

        event.send(DropMoney);
    }
}

fn drop_money_rain(
    mut event: EventReader<DropMoney>,
    field: Res<Field>,
    mut spawn_money_event: EventWriter<SpawnMoney>,
    mut drop_seed_event: EventWriter<DropSeed>,
) {
    for _ in event.read() {
        let mut rng = rand::thread_rng();
        let field_bounds = field.plants_bounds;
        let x_range = field_bounds.min.x..field_bounds.max.x;
        let y_range = field_bounds.min.y..field_bounds.max.y;

        let x = rng.gen_range(x_range);
        let y = rng.gen_range(y_range);

        let end_position = Vec3 { x, y, z: 0.0 };
        let mut start_position = end_position;
        start_position.y += constants::MONEY_RAIN_DROPLET_OFFSET_Y;

        let proc = rng.gen_range(0.0..=1.0) <= constants::SPAWN_SEED_CHANCE;
        if !proc {
            spawn_money_event.send(SpawnMoney { start_position, end_position });
        } else {
            drop_seed_event.send(DropSeed { start_position, end_position });
        }
    }
}

fn spawn_money(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut event: EventReader<SpawnMoney>,
) {
    for e in event.read() {
        commands.spawn(Name::new("money droplet"))
            .insert(MoneyDroplet)
            .insert(SpriteBundle {
                texture: asset_server.load("sprites/RoByn_small.png"),
                ..default()
            })
            .insert(Transform::from_translation(e.start_position).with_scale(Vec3::splat(0.2)))
            .insert(MoveToTarget(e.end_position))
            .insert(MovementSpeed(constants::MONEY_FALL_SPEED))
            .insert(TimeToLive(Timer::from_seconds(constants::MONEY_TTL, TimerMode::Once)))
            .insert(OnAppState(AppState::Gameplay))
            .insert(Clickable)
            .insert(CircleCollider::new(30.0))
        ;
    }
}

fn pick_money_droplet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut event: EventReader<Clicked>,
    coins: Query<Entity, With<MoneyDroplet>>,
    mut money: Query<&mut CurrentMoney>,
) {
    for e in event.read() {
        if let Ok(coin) = coins.get(e.0) {
            let mut money = money.single_mut();
            money.0 += 1;

            commands.entity(coin).despawn_recursive();

            commands.spawn(
                AudioBundle {
                    source: asset_server.load("audio/pop.ogg"),
                    settings: PlaybackSettings::DESPAWN.with_volume(Volume::new(0.1)),
                }
            );
        }
    }
}