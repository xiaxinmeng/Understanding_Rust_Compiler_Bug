rust
// Crate B - src/lib.rs
pub struct Two;

impl Two {
    pub fn new() -> Self { todo!() }
}

pub trait One {
    /// Do something with [`Two::new`]
    fn do_something(two: Two);
}
