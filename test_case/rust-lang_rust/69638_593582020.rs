rust
fn check<D: std::fmt::Debug>(constraints: impl Iterator<Item = D>) {
    for constraint in constraints {
        println!("{:?}", constraint);
    }
}
