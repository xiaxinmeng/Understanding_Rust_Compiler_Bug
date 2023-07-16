rust
// bin/okay.rs
use mycrate::Num;
use mycrate::Num6;

pub fn main() {
    let n = Num::<3>;
    n.foo();
    n.three();

    let n = Num::<1>;
    n.foo();

    let n = Num::<4>;
    n.foo();
    n.also_four();

    let n = Num::<5>;
    n.foo();

    let n = Num6::new();
    n.six();
}
