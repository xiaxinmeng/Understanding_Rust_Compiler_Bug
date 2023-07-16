rust
#![feature(existential_type)]
pub existential type X: FnOnce();
pub fn foo() -> X {
    || ()
}
