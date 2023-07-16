rust
struct Foo(Bar);
struct Bar;

trait BorrowFn<'a> {}
impl<'b, T: 'b, F: Fn(&'b Foo) -> T> BorrowFn<'b> for F {}

fn something<F>(_: F) where for<'c> F: BorrowFn<'c> {}

fn foo_as_bar<'d>(x: &'d Foo) -> &'d Bar { &x.0 }

fn main() {
    something(foo_as_bar);
}
