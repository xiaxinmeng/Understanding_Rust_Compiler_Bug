rust
fn foo() {
    #[derive(PartialEq, Clone, Copy)] // `x` is by-value-copy ==> OK today.
    #[derive(PartialEq, Clone)] // `x` is by-value-move ==> ERROR today.
    struct S;

    match S {
        x // `x` is by-value.
            if x == S => drop(x),
        _ => {}
    }
}
