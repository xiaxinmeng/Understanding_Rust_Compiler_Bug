rust
#![feature(nll)]

fn main() {
    let (x,) = {
        let z = 0;
        (&z,)
    };
}
