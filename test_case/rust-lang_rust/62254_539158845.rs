rust
fn foo(x: &[u32]) {
    match x {
        [..tail] => {} // error
    }
}
