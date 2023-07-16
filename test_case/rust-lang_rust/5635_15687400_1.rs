 rust
macro_rules! vec2_type(
    ($name:ident < $T:ty >) => (
        type $name = Vec2<$T>;

        impl $name {
            wrap_fn!(Vector2::new(x: $T ,y: $T) -> $name)
            wrap_fn!(Vector::from_value(v: $T) -> $name)
            wrap_fn!(NumericVector::identity() -> $name)
            wrap_fn!(NumericVector::zero() -> $name)
            wrap_fn!(NumericVector2::unit_x() -> $name)
            wrap_fn!(NumericVector2::unit_y() -> $name)

            wrap_fn!(dim() -> uint { 2 })
            wrap_fn!(size_of() -> uint { size_of::<$name>() })
        }
    )
)

pub vec2_type!(vec2<f32>)
pub vec2_type!(dvec2<f64>)
pub vec2_type!(ivec2<i32>)
pub vec2_type!(uvec2<u32>)
