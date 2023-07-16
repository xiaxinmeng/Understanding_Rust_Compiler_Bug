rust
#[derive(PartialEq, Eq)]
enum Nat {
    Z,
    S(Box<Nat>)
}

fn foo<const N: Nat>() {}
