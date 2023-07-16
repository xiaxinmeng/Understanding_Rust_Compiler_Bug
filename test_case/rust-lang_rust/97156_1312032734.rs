rust
fn takes_a_u(_: U) {}

fn takes_a_t(t: T) {
    // `t: T` coerced to supertype `U`
    takes_a_u(t);
}
