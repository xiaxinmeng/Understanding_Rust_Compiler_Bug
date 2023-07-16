rust
fn blah() -> impl for<'a> Trait<'a, Assoc = impl Sized + 'a> {
