rust
macro_rules! column { (foo) => { panic!() } }

fn main() {
    column!(foo);
}
