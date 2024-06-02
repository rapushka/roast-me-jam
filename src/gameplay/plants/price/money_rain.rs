use std::time::Duration;
use bevy::prelude::*;
use rand::Rng;
use crate::{AppState, constants};
use crate::gameplay::field::Field;
use crate::gameplay::movement::move_to_target::MoveToTarget;
use crate::gameplay::movement::MovementSpeed;
use crate::gameplay::plants::time_to_live::TimeToLive;

#[derive(Event)]
pub struct DropMoney;

#[derive(Resource)]
pub struct MoneyRain(pub Timer);

pub struct MoneyRainPlugin;

impl Plugin for MoneyRainPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<DropMoney>()

            .add_systems(OnEnter(AppState::Gameplay), start_money_rain)
            .add_systems(Update, (
                tick_money_rain,
                drop_money_rain,
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
    mut commands: Commands,
    mut event: EventReader<DropMoney>,
    asset_server: Res<AssetServer>,
    field: Res<Field>,
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

        commands.spawn(Name::new("money droplet"))
            .insert(SpriteBundle {
                texture: asset_server.load("sprites/RoByn.png"),
                ..default()
            })
            .insert(Transform::from_translation(start_position).with_scale(Vec3::splat(0.2)))
            .insert(MoveToTarget(end_position))
            .insert(MovementSpeed(constants::MONEY_FALL_SPEED))
            .insert(TimeToLive(Timer::from_seconds(constants::MONEY_TTL, TimerMode::Once)))
        ;
    }
}