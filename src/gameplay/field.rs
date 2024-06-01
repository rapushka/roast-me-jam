use std::ops::Range;
use bevy::math::bounding::Aabb2d;
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::constants;

#[derive(Resource)]
pub struct Field {
    pub zombies_spawn_x: f32,
    pub zombie_spawn_y_range: Range<f32>,
    /// if zombie reach this point - player loose
    pub player_house_x: f32,
    pub plants_bounds: Aabb2d,
    pub screen_center: Vec3,
    pub seed_slots_position: Vec2,
}

pub fn init_field(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window = windows.get_single().unwrap();
    let center_x = window.width() / 2.0;
    let center_y = window.height() / 2.0;

    let bounds = Aabb2d { min: Vec2::new(175.0, 120.0), max: Vec2::new(910.0, 560.0) };
    let screen_center = Vec3::new(center_x, center_y, 0.0);

    let seed_slots_position = Vec2::new(50.0, window.height() - 75.0);
    
    commands.insert_resource(Field {
        zombies_spawn_x: window.width() + constants::ZOMBIE_SPAWN_POINT_OFFSET,
        zombie_spawn_y_range: Range { start: bounds.min.y, end: bounds.max.y },
        player_house_x: 0.0 - constants::ZOMBIE_SPAWN_POINT_OFFSET,
        plants_bounds: bounds,
        screen_center,
        seed_slots_position,
    });
}
