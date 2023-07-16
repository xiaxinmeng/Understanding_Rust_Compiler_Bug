rust
fn takes_impl_trait(_: impl Trait) {}

fn takes_a_t(t: T) {
    // Compile error!
    // `t: T` not coerced to supertype `U`
    takes_impl_trait(t);
}
