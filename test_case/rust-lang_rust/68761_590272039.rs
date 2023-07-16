rust
pub fn foo(x: Option<std::num::NonZeroU32>) -> u32 {
    x.map_or(0, |n| n.get())
}
