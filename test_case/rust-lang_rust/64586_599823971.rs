rust
fn foo(x: &S) {
    let _: u32 = match x {
        S(mut y) => y,
        _ => 0,
    };
}
