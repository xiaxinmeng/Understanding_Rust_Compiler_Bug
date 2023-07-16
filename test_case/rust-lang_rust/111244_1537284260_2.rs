rust
fn from(other: Vec<T, A>) -> Self {
    let (ptr, len, cap, alloc) = other.into_raw_parts_with_alloc();
    Self { head: 0, len, buf: unsafe { RawVec::from_raw_parts_in(ptr, cap, alloc) } }
}
