rust
fn foo() -> &'static [u8] {
    // ok
    &[0, 1, 2, 3]
}

const fn bar() -> &'static [u8] {
    // error[E0723]: unsizing casts are not allowed in const fn (see issue #57563)
    // warning[E0515]: cannot return reference to temporary value
    &[0, 1, 2, 3]
}
