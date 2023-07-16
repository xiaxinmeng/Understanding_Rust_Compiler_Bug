rust
#![feature(existential_type)]

trait MyEq {
    fn my_eq(&self, other: &Self) -> bool;
}

existential type Bar: PartialEq + Eq;

fn bazr(x: u32) -> Bar {
    x
}

impl MyEq for Bar {
    fn my_eq(&self, other: &Self) -> bool {
        self == other
    }
}

fn main() {
    let x = bazr(42);
    let y = bazr(99);
    let z = bazr(42);
    assert!(x.my_eq(&z));
    assert!(!x.my_eq(&y));
    assert!(z.my_eq(&x));
}
