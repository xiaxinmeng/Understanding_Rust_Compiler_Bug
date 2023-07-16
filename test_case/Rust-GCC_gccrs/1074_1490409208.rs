rust
fn foo() {
    let a = { something }.len; // ok

    { something }.len; // KO
}
