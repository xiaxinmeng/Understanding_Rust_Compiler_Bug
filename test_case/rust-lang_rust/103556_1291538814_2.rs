rust
pub fn demo_obvious(x: &NonZeroU32, y: &NonZeroU32) -> bool {
    x.get() == y.get()
}
