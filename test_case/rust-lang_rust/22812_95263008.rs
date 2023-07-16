 rust
trait Foo {
    fn foo( &self );
}

struct Bar;
impl Foo for Bar {
    fn foo( &self ) { ; }
}

enum Baz {
    Baz(Bar)
}

impl std::ops::Deref for Baz {
    type Target = Foo;
    fn deref(&self) -> &Self::Target {
        match self {
            &Baz::Baz( ref t ) => t
        }
    }
}

impl std::ops::DerefMut for Baz {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            &mut Baz::Baz( ref mut t ) => t
        }
    }
}
fn main() {
  // Add code here
}

