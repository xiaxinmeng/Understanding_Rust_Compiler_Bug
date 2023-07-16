rust
#![feature(nll)]

fn foo<'r>(_: &'r ()) -> &'static () { &() }

fn main() {
    let f: for<'r> fn(&'r ()) -> &'r () = foo;
}
