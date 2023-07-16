rust
#![feature(optin_builtin_traits)]
#![feature(trait_alias)]

trait A<T: Send> {}
trait B<T: Send> = A<T>;

struct Foo<T>(T);
struct Bar();

impl<T: Send> A<T> for Foo<T> {}

impl !Send for Bar {}

fn main() {
    let b: Box<B<Bar>> = Box::new(Foo(Bar()));
    let a: Box<A<Bar>> = b; // should work
}
