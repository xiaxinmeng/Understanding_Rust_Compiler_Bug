rust
pub fn foo(g: fn()) {
    let _var = String::new(); // This does not alloc.
    g();
}
