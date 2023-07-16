rust
fn foo(x: Wibble<'_>) {}
fn bar(x: impl Quux<Wibble<'_>>) {}

trait Quux<T> {}
struct Wibble<'a>(&'a ());
