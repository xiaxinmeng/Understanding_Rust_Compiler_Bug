rust
pub fn f() -> impl Fn() { // OK
    || {} // It's hard to come up with something more private than a nameless fn-local closure.
}
