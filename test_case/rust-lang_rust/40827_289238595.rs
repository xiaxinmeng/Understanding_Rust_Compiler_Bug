rust
use std::rc::Rc;
use std::sync::Arc;

// Rc<_>: !Send
// => Bar: !Send
// => Arc<Bar>: !Send
// => Foo: !Send
//
// Shouldn't the compiler come to the same conclusion instead of entering this
// infinite recursion?
struct Foo(Arc<Bar>);

enum Bar {
    A(Rc<Foo>),
    B(Option<Foo>),
}

fn f<T: Send>(_: T) {}

fn main() {
    f(Foo(Arc::new(Bar::B(None))));
}
