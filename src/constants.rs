use std::ops::Range;

pub mod color;
pub mod styles;

pub mod controls {
    use bevy::prelude::KeyCode;

    pub const PAUSE_KEY: KeyCode = KeyCode::Escape;
}

pub const ZOMBIE_SPAWN_POINT_OFFSET: f32 = 75.0;
pub const PLAYER_HOUSE_POINT_OFFSET: f32 = 50.0;

pub const CASUAL_ZOMBIE_MOVEMENT_SPEED: Range<f32> = 20.0..50.0; 