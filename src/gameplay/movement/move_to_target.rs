use bevy::prelude::*;
use crate::gameplay::movement::MovementSpeed;

#[derive(Component)]
pub struct MoveToTarget(pub Vec3);

pub fn move_to_target(
    mut commands: Commands,
    mut entities: Query<(Entity, &mut Transform, &MoveToTarget, &MovementSpeed)>,
    time: Res<Time>,
) {
    for (e, mut transform, target, speed) in entities.iter_mut() {
        let direction = target.0 - transform.translation;
        let direction_normalized = direction.normalize_or_zero();

        if direction.length() <= 1.0 {
            transform.translation = target.0;
            commands.entity(e).remove::<MoveToTarget>();

            continue;
        }

        transform.translation += direction_normalized * speed.0 * time.delta_seconds();
    }
}