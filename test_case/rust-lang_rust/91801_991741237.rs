rust
trait Foo<'a> {}

fn or<'a>(first: &'static dyn Foo<'a>) -> dyn Foo<'a> {
    return Box::new(panic!());
}
