rust
fn check<I: Iterator>(constraints: I) where <I as Iterator>::Item: std::fmt::Debug {
    for constraint in constraints {
        println!("{:?}", constraint);
    }
}
