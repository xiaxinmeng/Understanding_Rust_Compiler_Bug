 rust
enum Foo { NoDrop, NeedsDrop(Box<int>) }
impl Drop for Foo {
    fn drop(&mut self) {
        *self = NeedsDrop(box 3);
    }
}
