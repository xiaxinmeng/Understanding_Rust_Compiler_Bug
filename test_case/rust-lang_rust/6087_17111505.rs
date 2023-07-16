 rust
macro_rules! vec3_type(
    ($name:ident <bool>) => (
        pub impl $name {
            #[inline(always)] fn new(x: bool, y: bool, z: bool) -> $name { BaseVec3::new(x, y, z) }
            #[inline(always)] fn from_value(v: bool) -> $name { BaseVec::from_value(v) }

            #[inline(always)] fn dim() -> uint { 3 }
            #[inline(always)] fn size_of() -> uint { sys::size_of::<$name>() }
        }
    );
    ($name:ident <$T:ty>) => (
        pub impl $name {
            #[inline(always)] fn new(x: $T, y: $T, z: $T) -> $name { BaseVec3::new(x, y, z) }
            #[inline(always)] fn from_value(v: $T) -> $name { BaseVec::from_value(v) }
            #[inline(always)] fn identity() -> $name { NumVec::identity() }
            #[inline(always)] fn zero() -> $name { NumVec::zero() }

            #[inline(always)] fn unit_x() -> $name { NumVec3::unit_x() }
            #[inline(always)] fn unit_y() -> $name { NumVec3::unit_y() }
            #[inline(always)] fn unit_z() -> $name { NumVec3::unit_z() }

            #[inline(always)] fn dim() -> uint { 3 }
            #[inline(always)] fn size_of() -> uint { sys::size_of::<$name>() }
        }
    );
)

// GLSL-style type aliases, corresponding to Section 4.1.5 of the [GLSL 4.30.6 specification]
// (http://www.opengl.org/registry/doc/GLSLangSpec.4.30.6.pdf).

// a three-component single-precision floating-point vector
pub type vec3  = Vec3<f32>;
// a three-component double-precision floating-point vector
pub type dvec3 = Vec3<f64>;
// a three-component Boolean vector
pub type bvec3 = Vec3<bool>;
// a three-component signed integer vector
pub type ivec3 = Vec3<i32>;
// a three-component unsigned integer vector
pub type uvec3 = Vec3<u32>;

vec3_type!(vec3<f32>)
vec3_type!(dvec3<f64>)
vec3_type!(bvec3<bool>)
vec3_type!(ivec3<i32>)
vec3_type!(uvec3<u32>)

// Rust-style type aliases
pub type Vec3f   = Vec3<float>;
pub type Vec3f32 = Vec3<f32>;
pub type Vec3f64 = Vec3<f64>;
pub type Vec3i   = Vec3<int>;
pub type Vec3i8  = Vec3<i8>;
pub type Vec3i16 = Vec3<i16>;
pub type Vec3i32 = Vec3<i32>;
pub type Vec3i64 = Vec3<i64>;
pub type Vec3u   = Vec3<uint>;
pub type Vec3u8  = Vec3<u8>;
pub type Vec3u16 = Vec3<u16>;
pub type Vec3u32 = Vec3<u32>;
pub type Vec3u64 = Vec3<u64>;
pub type Vec3b   = Vec3<bool>;

vec3_type!(Vec3f<float>)
vec3_type!(Vec3f32<f32>)
vec3_type!(Vec3f64<f64>)
vec3_type!(Vec3i<int>)
vec3_type!(Vec3i8<i8>)
vec3_type!(Vec3i16<i16>)
vec3_type!(Vec3i32<i32>)
vec3_type!(Vec3i64<i64>)
vec3_type!(Vec3u<uint>)
vec3_type!(Vec3u8<u8>)
vec3_type!(Vec3u16<u16>)
vec3_type!(Vec3u32<u32>)
vec3_type!(Vec3u64<u64>)
vec3_type!(Vec3b<bool>)
