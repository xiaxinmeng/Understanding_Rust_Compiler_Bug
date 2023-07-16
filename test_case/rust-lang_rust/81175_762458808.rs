rust
trait Foo<T> {
    const SIZE: usize = core::mem::size_of::<Self>();
}
