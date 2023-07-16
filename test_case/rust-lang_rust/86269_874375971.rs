rust
pub trait Lerp<T> {
    type Output;
    fn lerp(self, t: T) -> Self::Output;
}

impl Lerp<f32> for [f32; 2] {
    type Output = f32;
    fn lerp(self, t: f32) -> Self::Output {
        (1.0 - t) * self[0] + t * self[1]
    }
}

impl Lerp<f32> for [f32; 3] {
    type Output = f32;
    /// quadratic bezier curve -- a lerp of lerps
    fn lerp(self, t: f32) -> Self::Output {
        let nt = 1.0 - t;
        nt * nt * self[0] + nt * t * 2.0 * self[1] + t * t * self[2]
    }
}
