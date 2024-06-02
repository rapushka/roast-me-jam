use bevy::prelude::Component;

#[derive(PartialEq, Clone, Copy)]
pub enum EnemyType {
    Casual,
    Cone,
    Bucked,
}

#[derive(Component)]
pub struct Enemy(pub EnemyType);

