rust
pub trait CastInto<T> {
    fn cast_into(v: Self) -> T;
}

impl CastInto<u8> for u32 {
    #[inline]
    fn cast_into(v: u32) -> u8 {
        v as u32
    }
}
