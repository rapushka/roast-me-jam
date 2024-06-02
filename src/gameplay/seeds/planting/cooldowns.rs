use bevy::prelude::*;
use crate::AppState;

#[derive(Event)]
pub struct RestartCooldown(pub Entity);

#[derive(Component)]
pub struct CooldownTime(pub f32);

#[derive(Component)]
pub struct Cooldown(pub Timer);

pub struct CooldownsPlugin;

impl Plugin for CooldownsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<RestartCooldown>()

            .add_systems(Update, restart_cooldown.run_if(on_event::<RestartCooldown>()))
            .add_systems(Update, tick_cooldowns.run_if(in_state(AppState::Gameplay)))
        ;
    }
}

fn restart_cooldown(
    mut commands: Commands,
    mut event: EventReader<RestartCooldown>,
    cooldowns: Query<(Entity, &CooldownTime)>,
) {
    for e in event.read() {
        if let Ok((e, cd)) = cooldowns.get(e.0) {
            commands.entity(e)
                .insert(Cooldown(Timer::from_seconds(cd.0, TimerMode::Once)));
        }
    }
}

fn tick_cooldowns(
    mut commands: Commands,
    mut cooldowns: Query<(Entity, &mut Cooldown)>,
    time: Res<Time>,
) {
    for (e, cd) in cooldowns.iter_mut() {
        cd.0.tick(time.delta());

        if cd.0.finished() {
            commands.entity(e).remove::<Cooldown>();
        }
    }
}