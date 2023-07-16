rust
#![feature(const_let)]

const COND: bool = true;

const X: bool = {
    let mut ret = true;

    // If COND is true, we short circuit and ret remains true.
    // If COND is false, we assign ret = false.
    let _ = COND || { ret = false; true };

    ret
};

fn main() {
    println!("{}", X);
}
