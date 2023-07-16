rust
// Placeholder for whatever bounds you would want to write on the
// associated type.
trait Bounds: Default + std::fmt::Debug {}
impl<T> Bounds for T where T: Default + std::fmt::Debug {}

trait MyTrait {
    type Associated: Bounds = ();
    //~~~~~~~~~~~~~~~~~~~~~~^^^^ unstable
}

struct T1;
impl MyTrait for T1 {}

struct T2;
impl MyTrait for T2 {
    type Associated = String;
}

fn main() {
    println!("{:?}", T1::Associated::default());
    println!("{:?}", T2::Associated::default());
}
