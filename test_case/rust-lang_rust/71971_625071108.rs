rust
#![feature(const_fn)]
#![feature(const_trait_impl)]
#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl const std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, b: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + b.x,
            y: self.y + b.y,
            z: self.z + b.z,
        }
    }
}
