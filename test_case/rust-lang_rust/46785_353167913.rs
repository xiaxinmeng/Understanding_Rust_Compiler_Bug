rust
trait Trait {}
struct Foo<T, U>(T, U);
impl Trait for Foo<i32, i32> {}

struct Bogus<T = i32, U = i32>(Foo<T, U>) where Foo<T, U> : Trait;
