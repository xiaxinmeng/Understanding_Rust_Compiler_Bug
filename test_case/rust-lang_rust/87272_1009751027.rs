rust
struct S<'s, T: ToOwned + ?Sized>(&'s T);
fn bar<'s, T: ?Sized>(_: S<'s, T>) {}
