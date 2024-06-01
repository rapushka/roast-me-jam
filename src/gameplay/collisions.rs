use bevy::math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume};
use bevy::math::Vec3A;
use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use crate::gameplay::collisions::components::Collider;

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
    colliders: Query<(Entity, &Aabb, &GlobalTransform), (With<Collider>)>,
) {
    for (left, left_aabb, left_transform) in colliders.iter() {
        for (right, right_aabb, right_transform) in colliders.iter() {
            if left == right {
                continue;
            }

            let left_aabb = to_aabb_2d(left_aabb, left_transform);
            let right_aabb = to_aabb_2d(right_aabb, right_transform);

            // let right_aabb = Aabb2d { min: right_aabb.min().as_vec2(), max: right_aabb.max().as_vec2() };

            if left_aabb.contains(&right_aabb) {
                println!("{:?} hit {:?}", left, right);
            }
        }
    }
}

fn to_aabb_2d(aabb: &Aabb, transform: &GlobalTransform) -> Aabb2d {
    let min: Vec3 = aabb.min().into();
    let min = min + transform.translation();
    let max: Vec3 = aabb.max().into();
    let max = max + transform.translation();

    Aabb2d { min: min.as_vec2(), max: max.as_vec2() }
}

pub trait Vec3Ext {
    fn as_vec2(&self) -> Vec2;
}

impl Vec3Ext for Vec3A {
    fn as_vec2(&self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.y,
        }
    }
}

impl Vec3Ext for Vec3 {
    fn as_vec2(&self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.y,
        }
    }
}