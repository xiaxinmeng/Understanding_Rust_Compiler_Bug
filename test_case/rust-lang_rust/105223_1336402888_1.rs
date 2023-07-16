rust
use std::ops;

struct Bar;
impl ops::Neg for Bar {
    type Output = ();

    fn neg(self) {
        println!("neg");
    }
}
impl ops::Sub<Bar> for () {
    type Output = ();

    fn sub(self, _: Bar) {
        println!("sub");
    }
}

fn main() {
    {
        match () {
            () => (),
        } - match () {
            () => Bar,
        }
    }

    {
        (match () {
            () => (),
        }) - match () {
            () => Bar,
        }
    }
}
