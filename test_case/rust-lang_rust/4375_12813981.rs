
pub trait Cast {
    static fn from<T:CastTo<self>>(x: T) -> self;
}

pub trait CastTo<T> {
    fn cast(&self) -> T;
}

macro_rules ! impl_casts {
    {$from:ty -> $($to:ty),+} => {
        pub impl $from: Cast {
            #[inline(always)] static fn from<T:CastTo<$from>>(x: T) -> $from { x.cast() }
        }

        $(
            pub impl $from: CastTo<$to> {
                #[inline(always)] fn cast(&self) -> $to { (*self) as $to }
            }
        )+
    }
}

impl_casts! { u8    -> u8, u16, u32, u64, uint, i8, i16, i32, i64, int, f32, f64, float }
impl_casts! { u16   -> u8, u16, u32, u64, uint, i8, i16, i32, i64, int, f32, f64, float }
impl_casts! { u32   -> u8, u16, u32, u64, uint, i8, i16, i32, i64, int, f32, f64, float }
impl_casts! { u64   -> u8, u16, u32, u64, uint, i8, i16, i32, i64, int, f32, f64, float }
impl_casts! { uint  -> u8, u16, u32, u64, uint, i8, i16, i32, i64, int, f32, f64, float }
impl_casts! { i8    -> u8, u16, u32, u64, uint, i8, i16, i32, i64, int, f32, f64, float }
impl_casts! { i16   -> u8, u16, u32, u64, uint, i8, i16, i32, i64, int, f32, f64, float }
impl_casts! { i32   -> u8, u16, u32, u64, uint, i8, i16, i32, i64, int, f32, f64, float }
impl_casts! { i64   -> u8, u16, u32, u64, uint, i8, i16, i32, i64, int, f32, f64, float }
impl_casts! { int   -> u8, u16, u32, u64, uint, i8, i16, i32, i64, int, f32, f64, float }
impl_casts! { f32   -> u8, u16, u32, u64, uint, i8, i16, i32, i64, int, f32, f64, float }
impl_casts! { f64   -> u8, u16, u32, u64, uint, i8, i16, i32, i64, int, f32, f64, float }
impl_casts! { float -> u8, u16, u32, u64, uint, i8, i16, i32, i64, int, f32, f64, float }
