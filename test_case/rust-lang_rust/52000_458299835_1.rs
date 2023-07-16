
trait Bar {
    const BAR: Self;
}
trait Foo<T: Bar> {
    const FOO: u32 = (T::BAR, 42).1;      // Doesn't this imply either `T: const Drop` or `T: !Drop`?
}
