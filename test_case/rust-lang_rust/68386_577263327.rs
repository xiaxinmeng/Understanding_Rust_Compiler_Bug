rust
fn foo() -> u32 {
    let a = {
        let b = 5;
        (b)
    };
    (a)
}
