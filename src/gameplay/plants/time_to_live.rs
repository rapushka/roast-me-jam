use bevy::prelude::*;

use crate::gameplay::health::Kill;

#[derive(Component)]
pub struct TimeToLive(pub Timer);

pub fn update_time_to_live(
    mut entities: Query<(Entity, &mut TimeToLive)>,
    time: Res<Time>,
    mut kill_request: EventWriter<Kill>,
) {
    for (e, mut ttl) in entities.iter_mut() {
        let mut timer = &mut ttl.0;
        timer.tick(time.delta());

        if timer.finished() {
            kill_request.send(Kill(e));
        }
    }
}