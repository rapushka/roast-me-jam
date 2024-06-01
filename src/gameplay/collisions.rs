use bevy::math::bounding::{BoundingVolume, IntersectsVolume};
use bevy::prelude::*;

use crate::gameplay::collisions::components::*;

pub mod components;

#[derive(Event)]
pub struct Collision(pub Entity, pub Entity);

pub struct CollisionsPlugin;

impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<Collision>()

            .add_systems(FixedUpdate, check_collisions)
        ;
    }
}

fn check_collisions(
    colliders: Query<(Entity, &GlobalTransform, &CircleCollider)>,
    mut event_writer: EventWriter<Collision>,
) {
    for (left, left_transform, left_collider) in colliders.iter() {
        for (right, right_transform, right_collider) in colliders.iter() {
            if left == right {
                continue;
            }

            let left_position = left_transform.translation();
            let right_position = right_transform.translation();

            let distance = left_position.distance(right_position);

            if distance <= left_collider.radius
                || distance <= right_collider.radius {
                event_writer.send(Collision(left, right));
            }
        }
    }
}