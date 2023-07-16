rust
#![feature(adt_const_params)]

#[derive(PartialEq, Eq)]
enum Nat {
    Z,
    S(Box<Nat>)
}

fn foo<const N: Nat>() {}
