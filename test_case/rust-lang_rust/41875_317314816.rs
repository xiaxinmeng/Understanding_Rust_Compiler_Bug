rust
pub fn type_id<T: ?Sized>() -> u64 {
    type_id::<T> as usize as u64
}
