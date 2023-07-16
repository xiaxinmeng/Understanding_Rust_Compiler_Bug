rust
fn not_working<V>(a: f32, b: f32, v: &V) -> V where
    f32: core::ops::Mul<f32, Output=f32>,
    f32: for<'a> core::ops::Mul<&'a V, Output=V>,
{
    a * b * v
}
