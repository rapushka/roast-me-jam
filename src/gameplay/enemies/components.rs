use bevy::prelude::Component;

#[derive(PartialEq, Clone, Copy)]
pub enum EnemyType {
    Casual,
}

#[derive(Component)]
pub struct Enemy(pub EnemyType);

