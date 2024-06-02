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

pub const ASH_BABY_TTL: f32 = 1.0;

// # Balance
pub const SEED_SLOT_COUNT: u8 = 3;
pub const START_MONEY: i32 = 5;
pub const MONEY_RAIN_FREQUENCY_RANGE: Range<f32> = 1.0..5.0; // in seconds

pub const DIFFICULTY_INCREMENT: f32 = 0.02;

// ## Money rain
pub const MONEY_RAIN_DROPLET_OFFSET_Y: f32 = 650.0;
pub const MONEY_FALL_SPEED: f32 = 100.0;
pub const MONEY_TTL: f32 = 10.0;
pub const SPAWN_SEED_CHANCE: f32 = 0.1;

// ## Zombie
pub const CASUAL_ZOMBIE_MOVEMENT_SPEED: Range<f32> = 20.0..40.0;
pub const CASUAL_ZOMBIE_HEALTH: f32 = 100.0;
pub const CONE_ZOMBIE_HEALTH: f32 = 150.0;
pub const BUCKET_ZOMBIE_HEALTH: f32 = 250.0;
pub const SPAWN_FIRST_ENEMY_DURATION: f32 = 5.0;
pub const SPAWN_ENEMY_DURATION_RANGE: Range<f32> = 8.0..10.0;

// ## Plants
// ### Fire
pub const FIRE_DAMAGE: f32 = 40.0;
pub const FIRE_TTL_IN_SECONDS: f32 = 2.0;
pub const FIRE_PRICE: i32 = 3;

// ### Money
pub const MONEY_PLANT_PRICE: i32 = 6;
pub const MONEY_PLANT_HARVEST: f32 = 6.0;
pub const MONEY_PLANT_SPAWN_MONEY_X: Range<f32> = -35.0..35.0;
pub const MONEY_PLANT_SPAWN_MONEY_Y: Range<f32> = 15.0..16.0;
pub const MONEY_PLANT_SPAWN_MONEY_FALL_DISTANCE: f32 = 50.0;

// ### Lego
pub const LEGO_PRICE: i32 = 2;
pub const LEGO_DAMAGE: f32 = 5.0;

// ### Skibidi toilet
pub const SKIBIDI_TOILET_PRICE: i32 = 5;
pub const SKIBIDI_TOILET_ROF: f32 = 1.0;

// ## Projectiles
// ### Nuke
pub const NUKE_MOVEMENT_SPEED: f32 = 100.0;
pub const NUKE_DAMAGE: f32 = 5.0;
