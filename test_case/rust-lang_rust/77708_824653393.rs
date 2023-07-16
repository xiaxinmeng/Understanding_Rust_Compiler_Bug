rust
#![feature(const_generics)]
#![feature(const_evaluatable_checked)]

struct Z;
const fn one() -> usize { 1 }

fn from_a_to_b<T>(source: [u8; one()]) -> T {
    todo!()
}

fn main() {
    let _: &Z = from_a_to_b([0; 1]);
}
