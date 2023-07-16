rust
use glam::{Quat, Vec3};

#[derive(Clone, Copy)]
pub struct Transform {
    position: Vec3,
    rotation: Quat,
    scale: Vec3,
}

impl Transform {
    pub const fn new() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }

    pub const fn with_position(mut self, position: Vec3) -> Self {
        self.position = position;
        self
    }
}
