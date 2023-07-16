 rust
trait Foo { fn foo(&self) -> Self; }

fn foo<T: Foo>(x: T) -> T { x.foo() }

fn main() {
    // foo(x) and x.foo() should be equivalent, but aren’t

    // Bad:
    let _ = foo(loop {}); // error: the trait `Foo` is not implemented for the type `()`
    // Good:
    let _ = loop {}.foo(); // error: the type of this value must be known in this context
}
