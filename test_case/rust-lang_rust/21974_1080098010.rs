rust
trait MyTrait {}
// The bound we want to meet
fn foo<T>(_: &T) where for<'any> &'any T: MyTrait {}

// Implemented with an explicit `'inner: 'outer` bound
struct Baz;
impl<'outer, 'inner: 'outer> MyTrait for &'outer &'inner Baz {}

fn main() {
    // `&'_ &'static Baz` meets the bounds
    foo(&&Baz);
    let local_static_ref = &Baz;
    foo(&local_static_ref);
    // `&'_ &'local Baz` does not
    let baz = Baz;
    foo(&&baz);
}
