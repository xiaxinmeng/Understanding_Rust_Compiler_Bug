 Rust
// Crate a:
struct A;
impl<T> Base for Pair<Option<T>, Option<A>> {}
// Crate b:
struct B;
impl<T> Base for Pair<Option<B>, Option<T>> {}
