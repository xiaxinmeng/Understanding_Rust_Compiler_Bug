rust
#![feature(nll)]

fn main() {
    let mut x = (&22,);

    {
        let y = 33;
        x.0 = &y;
        println!("{:?}", x);
    }

    {
        let z = 33;
        x.0 = &z;
        println!("{:?}", x);
    }
}
