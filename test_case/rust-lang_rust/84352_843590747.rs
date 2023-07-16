rust
#[cfg(target_pointer_width = "64")]
pub fn foo(x: u64, y: NonZeroUsize) -> u64 {
    x / (y.get() as u64)
}
