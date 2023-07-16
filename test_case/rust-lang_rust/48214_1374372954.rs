rust
trait Trait {}

struct Foo {
    inner: Option<Box<Foo>>,
}

impl Trait for Foo
where Option<Box<Foo>>: Trait // the problematic line
{}

impl<T: Trait> Trait for Option<T> {}
impl<T: Trait> Trait for Box<T> {}
