use std::time::Duration;
use bevy::prelude::*;
use rand::Rng;
use crate::{AppState, constants};

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
    money_rain: Res<MoneyRain>,
    mut event: EventReader<DropMoney>,
) {
    for _ in event.read() {
        println!("mone!!!!!!");
    }
}