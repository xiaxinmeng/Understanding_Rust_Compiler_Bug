rust
trait Foo<T> {
    const FOO: usize = div2_checked(std::mem::size_of::<T>());
}
