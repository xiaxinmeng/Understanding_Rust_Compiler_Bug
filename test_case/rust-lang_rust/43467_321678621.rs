rust
extern {
    type foo;
}
fn null_foo() -> *const foo {
    0usize as *const foo
}
