rust
fn main() {
    // I can't see any issues stemming from
    // overloading the meaning of labeled blocks.
    let x = 'a: {
        let mut args = std::env::args().skip(1);
        (&'a mut args).next().unwrap()
    };

    // 'x has to start here since `x: 'x` must hold
    'x: {
        // println! shouldn't implictly borrow its args. how confusing.
        let x_ref = &'x x;
        println!("hello {x_ref}!");
    }
}
