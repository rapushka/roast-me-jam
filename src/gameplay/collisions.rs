use bevy::math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume};
use bevy::math::Vec3A;
use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use crate::gameplay::collisions::components::*;

pub mod components;

pub struct CollisionsPlugin;

impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, check_collisions)
        ;
    }
}

fn check_collisions(
    colliders: Query<(Entity, &GlobalTransform, &CircleCollider)>,
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
                println!("{:?} hit {:?}", left, right);
            }
        }
    }
}