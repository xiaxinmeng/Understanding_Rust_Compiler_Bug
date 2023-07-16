 rust
trait ParamTrait {}

struct Param;

impl ParamTrait for Param {}

struct Bar;

trait Foo {
    fn test_fn<T:Eq>(&self, x: T, y: T);
}

impl Foo for Bar {
    // The type param of the implementation is not
    // the same as specified in the trait `Foo` nor implied
    // by it.
    fn test_fn<T: ParamTrait>(&self, x: T, y: T) {}
}

fn main() {
    let x: Bar = Bar;
    x.test_fn(Param, Param);
}
