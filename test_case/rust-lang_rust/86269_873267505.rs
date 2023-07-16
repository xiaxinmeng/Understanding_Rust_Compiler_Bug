rust
pub trait Lerp<R = Self, const N: usize = 2> {
    type Output;
    fn lerp(self, control_points: [R; N]) -> Self::Output;
}

impl Lerp for f32 {
    type Output = f32;
    fn lerp(self, control_points: [f32; 2]) -> f32 {
        (1.0 - self) * control_points[0] + self * control_points[1]
    }
}

impl Lerp<f32, 3> for f32 {
    type Output = f32;
    /// quadratic bezier curve -- a lerp of lerps
    fn lerp(self, control_points: [f32; 3]) -> f32 {
        let nt = 1.0 - self;
        nt * nt * control_points[0] + nt * self * 2.0 * control_points[1] + self * self * control_points[2]
    }
}
