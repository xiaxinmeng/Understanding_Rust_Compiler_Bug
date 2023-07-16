rust
pub struct Bar([u8]);

impl Bar {
    fn needs_sized(&self) where Self: Sized {}
}
