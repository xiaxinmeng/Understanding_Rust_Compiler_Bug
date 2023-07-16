rust
struct Foo { f: () }

impl std::ops::Deref for Foo {
    type Target = ();
    fn deref(&self) -> &() {
        &self.f
    }
}

trait A {}
impl A for () {}
fn foo<T: A>(f: &T) {}

fn main() {
    foo(&());
    foo(&Foo { f: () }); // Errors.
}
