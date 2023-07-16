rust
// For every function foo like
fn foo<'long>(r: &'long mut T) -> &'long mut T { ... }

// It should be possible to write a function bar like
fn bar<'short, 'long: 'short>(p: &'short mut &'long mut T) {
    *p = foo(*p);  // doesn't compile
}

// Since inlining bar is accepted.
fn main() {
    let mut x = Default::default();
    let mut r = &mut x;
    r = foo(r);  // compiles
    bar(&mut r);  // should be possible
}
