rust
fn main() {
    f::<S, Z>(S, Z); // ok
    f::<S, S>(S, S); // error[E0277]: the trait bound `(S, S): NotSame` is not satisfied
}

fn a<'a>() {
    f::<&'a (), &'static ()>(&(), &()); // error[E0277]: the trait bound `(&'a (), &'static ()): NotSame` is not satisfied
}
