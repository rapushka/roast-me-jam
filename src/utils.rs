use bevy::math::{Vec2, Vec3};

pub trait Vec2Ext {
    fn as_vec3(&self) -> Vec3;
}

impl Vec2Ext for Vec2 {
    fn as_vec3(&self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z: 0.0,
        }
    }
}
