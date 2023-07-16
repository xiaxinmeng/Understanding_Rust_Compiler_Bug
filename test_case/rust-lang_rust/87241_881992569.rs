rust
struct C<'a, 'b>(&'a &'b ());

// for<> with where bounds does not exist.
fn foo<'a>(f: impl for<'b: 'a> FnOnce(&mut C<'b, 'a>)) {}
fn bar<'a>(f: impl for<'b: 'a> FnOnce(&mut C<'b, 'a>)) {
    foo(|x| f(x))
}
