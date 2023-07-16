rust
static F: Lazy<impl FnOnce() -> u32> = Lazy::new(|| 1);
