rust
macro m() {
    pub struct A; // (1)
    A; // resolves to (1)
}

fn f() {
    m!();
    struct A; // (2)
    A; // resolves to (2)
}
