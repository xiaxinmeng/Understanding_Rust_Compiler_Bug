 rust
trait Data : Clone+Send+'static { }
impl<T: Clone+Send+'static> Data for T { }

pub trait UnaryLogic {
    type D: Data;
}

pub struct Test<D: Data> {
    data: D,
}

pub struct UnaryScope<L: UnaryLogic> {
    input: Test<L::D>,
}

fn main() {}
