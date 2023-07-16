rust
pub struct Foo {
    bar: Cell<Option<&'static mut u32>>,
}

impl Foo {
    pub const fn new() -> Self {
        Self {
            bar: Cell::new(None)
        }
    }
}
