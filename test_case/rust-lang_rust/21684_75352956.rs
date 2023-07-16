 rust
trait Foo<T> {
    fn frog(&self);
    fn turkey(&mut self) where T = u8;
}
