rust
trait Foo<T> {
    const X: T;
}

trait Bar<T, U: Foo<T>> {
    const F: u32 = (U::X, 42).1;
                   ^^^^^^^^^^ destructors in constants are not allowed
}
