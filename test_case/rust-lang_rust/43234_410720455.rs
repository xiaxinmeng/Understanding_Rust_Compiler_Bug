rust
#![feature(nll)]

fn main() {
    let mut x;

    {
        let y = 33;
        x = &y;
        println!("{:?}", x);
    }

    {
        let z = 33;
        x = &z;
        println!("{:?}", x);
    }
}
