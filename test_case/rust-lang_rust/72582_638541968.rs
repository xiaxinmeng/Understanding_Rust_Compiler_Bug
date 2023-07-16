rust
trait Foo: PartialOrd + PartialOrd<i32> {}

fn baz<T: Foo>(x: T, y: T) -> bool {
    x < y
}
