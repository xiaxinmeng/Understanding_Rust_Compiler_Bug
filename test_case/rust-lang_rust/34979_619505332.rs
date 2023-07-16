rust
trait Foo {}
impl<'a, T> Foo for &'a T {}

// we need `'a _` and `'static _` in the same
// `where` bound afaict.
struct Ctx<'a>(&'a ()) where
    &'a (): Foo,
    &'static (): Foo;
