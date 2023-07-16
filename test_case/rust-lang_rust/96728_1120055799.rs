rust
#![feature(generic_const_exprs)]
#![feature(inline_const)]
#![allow(incomplete_features)]

struct Foo;

impl Foo {
    fn foo<const N: usize>(&self) {

    }
}

fn bar<const N: usize>() {

}

struct Baz<const N: usize>;

impl<const N: usize> Baz<N> {
    const V: usize = N;
}

// This is a different kind of cycle that I don't understand.
impl Baz::<{let _ = || 4; 4}> {}

fn main() {
    // A cycle caused by the second match arm of opt_const_param_of
    let _ = Foo.foo::<{let _ = || 4; 4}>();
    // A cycle caused by the third match arm of opt_const_param_of
    // a forced check `let _tables = tcx.typeck(..)` (type_of.rs:137)
    let _ = bar::<{const {1}}>();
    // Same as above.
    let _ = Baz::<{const {1}}>;
    // Allowed, hit the first match arm of opt_const_param_of, which doesn't do typeck
    let _ = Baz::<{const {1}}>::V;
}
