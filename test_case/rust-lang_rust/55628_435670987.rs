rust
trait Foo<X> {}

type Bar<X: ExtraBound> = dyn Foo<X>;
fn bad<X>(_: &Bar<X>) {}

trait Foo2<X: ExtraBound> = Foo<X>;
fn bad2<X>(_: &dyn Foo2<X>) {}
