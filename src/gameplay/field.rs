use bevy::math::bounding::Aabb2d;
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::constants;

#[derive(Resource)]
pub struct Field {
    pub zombies_spawn_x: f32,
    /// if zombie reach this point - player loose
    pub player_house_x: f32,
    pub plants_bounds: Aabb2d,
    pub screen_center: Vec3,
}

pub fn init_field(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window = windows.get_single().unwrap();
    let center_x = window.width() / 2.0;
    let center_y = window.height() / 2.0;

    let screen_center = Vec3::new(center_x, center_y, 0.0);

    println!("{}", window.width());
    commands.insert_resource(Field {
        zombies_spawn_x: window.width() + constants::ZOMBIE_SPAWN_POINT_OFFSET,
        player_house_x: 0.0 - constants::ZOMBIE_SPAWN_POINT_OFFSET,
        plants_bounds: Aabb2d::new(Vec2::new(center_x, center_y), Vec2::new(center_x, center_y)),
        screen_center,
    });
}
