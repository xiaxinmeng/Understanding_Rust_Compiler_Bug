rust
fn foo() -> [A; 0] {
    let a = A;
    <[A; 0]>::new(a)
}
