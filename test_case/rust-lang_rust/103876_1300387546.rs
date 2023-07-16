rust
// ... same test as #102417
fn want_static<T: 'static>(_: T) {}

fn test<'a>() {
    want_static(<&'a () as Callable>::call());
}
