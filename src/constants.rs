use std::ops::Range;

pub mod color;
pub mod styles;
pub mod z_order;

pub mod controls {
    use bevy::prelude::KeyCode;

    pub const PAUSE_KEY: KeyCode = KeyCode::Escape;
}

pub const ZOMBIE_SPAWN_POINT_OFFSET: f32 = 75.0;
pub const PLAYER_HOUSE_POINT_OFFSET: f32 = 50.0;

// # Balance
pub const SEED_SLOT_COUNT: u8 = 3;
pub const START_MONEY: i32 = 5;

// ## Zombie
pub const CASUAL_ZOMBIE_MOVEMENT_SPEED: Range<f32> = 20.0..50.0;
pub const CASUAL_ZOMBIE_HEALTH: f32 = 100.0;

// ## Plants
// ### Fire
pub const FIRE_DAMAGE: f32 = 25.0;
pub const FIRE_TTL_IN_SECONDS: f32 = 2.0;
pub const FIRE_PRICE: i32 = 3;