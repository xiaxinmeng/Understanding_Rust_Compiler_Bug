rust
pub trait Lerp<T = [Self; 2]>: Sized {
    type Output;
    fn lerp(self, control_points: T) -> Self::Output;
}

impl Lerp<[Vec3; 2]> for f32 {
    type Output = Vec3;
    fn lerp(self, control_points: [Vec3; 2]) -> Self::Output {
        (1.0 - self) * control_points[0] + self * control_points[1]
    }
}