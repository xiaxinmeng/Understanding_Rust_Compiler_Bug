rust
trait Foo<Args> {
    type Output;
}

trait Bar<'a, T>: Foo<T, Output=bool> {
    fn cb(&self) -> Box<dyn Bar<'a, T, Output=bool>>;
}
